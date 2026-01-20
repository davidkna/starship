use super::{Context, Module, ModuleConfig};

use crate::configs::cmd_duration::CmdDurationConfig;
use crate::formatter::StringFormatter;
use crate::segment::Segment;
use crate::utils::render_time;

/// Outputs the time it took the last command to execute
///
/// Will only print if last command took more than a certain amount of time to
/// execute. Default is two seconds, but can be set by config option `min_time`.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("cmd_duration");
    let config: CmdDurationConfig = CmdDurationConfig::try_load(module.config);

    if config.min_time < 0 {
        log::warn!(
            "min_time in [cmd_duration] ({}) was less than zero",
            config.min_time
        );
        return None;
    }

    let elapsed = context.get_cmd_duration()?;
    let config_min = config.min_time as u128;

    if elapsed < config_min {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "duration" => Some(Ok(render_time(elapsed, config.show_milliseconds))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `cmd_duration`: \n{error}");
            return None;
        }
    });

    Some(undistract_me(module, &config, context, elapsed))
}


fn undistract_me<'a>(
    mut module: Module<'a>,
    config: &CmdDurationConfig,
    context: &'a Context,
    elapsed: u128,
) -> Module<'a> {
    use nu_ansi_term::{AnsiStrings, unstyle};

    if config.show_notifications && config.min_time_to_notify as u128 <= elapsed {
        let body = format!(
            "Command execution {}",
            unstyle(&AnsiStrings(&module.ansi_strings()))
        );

        // Try terminal notification via OSC sequences first
        let used_osc = add_terminal_notification(&mut module, context, &body, config.notification_timeout);

        // If terminal supports OSC notifications, skip native notification
        if used_osc {
            return module;
        }
    }

    // Don't send native notifications during tests if the env var is set
    if cfg!(test) {
        return module;
    }

    // Fall back to native notification for unsupported terminals
    if cfg!(target_os = "linux") {
        let in_graphical_session = ["DISPLAY", "WAYLAND_DISPLAY", "MIR_SOCKET"]
            .iter()
            .find_map(|&var| context.get_env(var).filter(|val| !val.is_empty()))
            .is_some();

        if !in_graphical_session {
            return module;
        };
    }

    if !(cfg!(feature = "notify")) {
        log::warn!(
            "Notifications are enabled in [cmd_duration], but the notify feature is not enabled"
        );
        return module;
    }

    #[cfg(feature = "notify")]
    send_native_notification(config, &module);

    module
}

enum OscType {
    Osc9,   // iTerm2
    Osc777, // Ghostty, WezTerm, Foot, urxvt, Warp
}

impl OscType {
    fn format(&self, body: &str) -> String {
        match self {
            OscType::Osc9 => format!("\x1b]9;{}\x07", body),
            OscType::Osc777 => format!("\x1b]777;notify;Command finished;{}\x1b\\", body),
        }
    }
}

fn add_terminal_notification(module: &mut Module, context: &Context, body: &str, timeout_ms: Option<u32>) -> bool {
    let term_program = context.get_env("TERM_PROGRAM").unwrap_or_default();

    // Detect terminal and determine OSC type (allowlist approach)
    let osc_type = match term_program.as_str() {
        "iTerm.app" => Some(OscType::Osc9),
        "ghostty" => Some(OscType::Osc777),
        "WezTerm" => Some(OscType::Osc777),
        "WarpTerminal" => Some(OscType::Osc777),
        // Kitty on Linux uses OSC 99
        "kitty" if cfg!(target_os = "linux") => Some(OscType::Osc777),
        // Kitty on other platforms uses OSC 777
        "kitty" => Some(OscType::Osc777),
        // Unsupported terminals (use native notification)
        _ => None,
    };

    if let Some(osc) = osc_type {
        module.segments.push(Segment::control(osc.format(body, timeout_ms)));
        true // OSC sequence was added
    } else {
        false // No OSC support, use native notification
    }
}

#[cfg(feature = "notify")]
fn send_native_notification(config: &CmdDurationConfig, module: &Module) {
    use notify_rust::{Notification, Timeout};

    // On macOS 26+ notify-rust will get stuck finding the current application identifier
    // so we set it manually to the default terminal app.
    #[cfg(target_os = "macos")]
    let _ = notify_rust::set_application("com.apple.Terminal");

    let timeout = match config.notification_timeout {
        Some(v) => Timeout::Milliseconds(v),
        None => Timeout::Default,
    };

    let body = format!(
        "Command execution {}",
        nu_ansi_term::AnsiStrings(&module.ansi_strings())
    );

    let mut notification = Notification::new();
    notification
        .summary("Command finished")
        .body(&body)
        .icon("utilities-terminal")
        .timeout(timeout);

    if let Err(err) = notification.show() {
        log::trace!("Cannot show notification: {err}");
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn config_blank_duration_1s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(1000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_duration_5s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("took {} ", Color::Yellow.bold().paint("5s")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_5s_duration_3s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 5000
            })
            .cmd_duration(3000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_5s_duration_10s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 5000
            })
            .cmd_duration(10000)
            .collect();

        let expected = Some(format!("took {} ", Color::Yellow.bold().paint("10s")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_1s_duration_prefix_underwent() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [$duration]($style) "
            })
            .cmd_duration(1000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_5s_duration_prefix_underwent() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [$duration]($style) "
            })
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("underwent {} ", Color::Yellow.bold().paint("5s")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_notifications_wezterm() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 1000
                show_notifications = true
                min_time_to_notify = 2000
            })
            .env("TERM_PROGRAM", "WezTerm")
            .cmd_duration(3000)
            .collect();

        let visible = format!("took {} ", Color::Yellow.bold().paint("3s"));
        let escape = "\x1b]777;notify;Command finished;Command execution took 3s \x1b\\";
        let expected = Some(format!("{}{}", visible, escape));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_notifications_iterm2() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 1000
                show_notifications = true
                min_time_to_notify = 2000
            })
            .env("TERM_PROGRAM", "iTerm.app")
            .cmd_duration(3000)
            .collect();

        let visible = format!("took {} ", Color::Yellow.bold().paint("3s"));
        let escape = "\x1b]9;Command execution took 3s \x07";
        let expected = Some(format!("{}{}", visible, escape));
        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn config_notifications_kitty_linux() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 1000
                show_notifications = true
                min_time_to_notify = 2000
            })
            .env("TERM_PROGRAM", "kitty")
            .cmd_duration(3000)
            .collect();

        let visible = format!("took {} ", Color::Yellow.bold().paint("3s"));
        let escape = "\x1b]99;i=1:d=0;Command execution took 3s \x1b\\";
        let expected = Some(format!("{}{}", visible, escape));
        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn config_notifications_kitty_linux_with_timeout() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 1000
                show_notifications = true
                min_time_to_notify = 2000
                notification_timeout = 5000
            })
            .env("TERM_PROGRAM", "kitty")
            .cmd_duration(3000)
            .collect();

        let visible = format!("took {} ", Color::Yellow.bold().paint("3s"));
        let escape = "\x1b]99;i=1:d=5000;Command execution took 3s \x1b\\";
        let expected = Some(format!("{}{}", visible, escape));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_notifications_ghostty() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 1000
                show_notifications = true
                min_time_to_notify = 2000
            })
            .env("TERM_PROGRAM", "ghostty")
            .cmd_duration(3000)
            .collect();

        let visible = format!("took {} ", Color::Yellow.bold().paint("3s"));
        let escape = "\x1b]777;notify;Command finished;Command execution took 3s \x1b\\";
        let expected = Some(format!("{}{}", visible, escape));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_notifications_unsupported_terminal() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 1000
                show_notifications = true
                min_time_to_notify = 2000
            })
            .env("TERM_PROGRAM", "UnsupportedTerminal")
            .cmd_duration(3000)
            .collect();

        // Unsupported terminals should not get OSC sequences (no escape sequence)
        let visible = format!("took {} ", Color::Yellow.bold().paint("3s"));
        let expected = Some(visible);
        assert_eq!(expected, actual);
    }
}
