use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use process_control::{ChildExt, Control, Output};
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store, StoreLimits, StoreLimitsBuilder};
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use super::{Context, Module, ModuleConfig};
use crate::{
    config::Either,
    configs::custom_wasm::CustomWasmConfig,
    formatter::{StringFormatter, VariableHolder},
    utils::create_command,
};

// Generate host bindings from WIT
wasmtime::component::bindgen!({
    path: "wit/starship-wasm.wit",
    world: "starship-module",
});

struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
    limits: StoreLimits,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

// Global engine instance (created once, reused)
static ENGINE: OnceLock<Engine> = OnceLock::new();

const WASM_MEMORY_LIMIT: usize = 64 * 1024 * 1024;

/// Creates a custom WASM module
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("custom_wasm");
    let config = CustomWasmConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    if let Some(os) = config.os
        && os != env::consts::OS
        && !(os == "unix" && cfg!(unix))
    {
        return None;
    }

    if config.require_repo && context.get_repo().is_err() {
        return None;
    }

    if config.wasm_path.is_empty() {
        log::warn!("custom_wasm module enabled but wasm_path is empty");
        return None;
    }

    // Run the cheap detection checks before touching the WASM runtime. Unlike
    // `custom` modules, a component carries its own `is-enabled` check, so when
    // no detection criteria are configured at all the component alone decides.
    let no_criteria = config.detect_files.is_empty()
        && config.detect_extensions.is_empty()
        && config.detect_folders.is_empty()
        && matches!(config.when, Either::First(false));

    if !no_criteria {
        let mut is_match = context
            .try_begin_scan()?
            .set_extensions(&config.detect_extensions)
            .set_files(&config.detect_files)
            .set_folders(&config.detect_folders)
            .is_match();

        if !is_match {
            is_match = match config.when {
                Either::First(b) => b,
                Either::Second(cmd) => exec_when(cmd, &config, context),
            };

            if !is_match {
                return None;
            }
        }
    }

    let wasm_path = Context::expand_tilde(PathBuf::from(config.wasm_path));

    if !wasm_path.exists() {
        log::warn!("WASM file not found: {}", wasm_path.display());
        return None;
    }

    let engine = ENGINE.get_or_init(|| {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(false);
        config.epoch_interruption(true);
        Engine::new(&config).expect("Failed to create WASM engine")
    });

    let component = match Component::from_file(engine, &wasm_path) {
        Ok(c) => c,
        Err(e) => {
            log::warn!("Failed to load WASM component: {e}");
            return None;
        }
    };

    let mut wasi_builder = WasiCtxBuilder::new();
    // stdout is deliberately not inherited: the component reports its output
    // through the `formatter` interface, and stray prints would corrupt the prompt
    wasi_builder.inherit_env().inherit_stderr();
    // Read-only view of the current directory, mounted at "."
    if let Err(e) = wasi_builder.preopened_dir(
        &context.current_dir,
        ".",
        DirPerms::READ,
        FilePerms::READ,
    ) {
        log::debug!("Failed to preopen current directory for WASM component: {e}");
    }

    let mut linker = Linker::new(engine);
    if let Err(e) = wasmtime_wasi::p2::add_to_linker_sync(&mut linker) {
        log::warn!("Failed to add WASI to linker: {e}");
        return None;
    }

    let mut store = Store::new(
        engine,
        HostState {
            wasi: wasi_builder.build(),
            table: ResourceTable::new(),
            limits: StoreLimitsBuilder::new()
                .memory_size(WASM_MEMORY_LIMIT)
                .build(),
        },
    );
    store.limiter(|state| &mut state.limits);

    // Trap any guest code still running once `command_timeout` has elapsed,
    // so a misbehaving component cannot hang the prompt.
    if config.ignore_timeout {
        store.set_epoch_deadline(u64::MAX);
    } else {
        store.set_epoch_deadline(1);
        let engine = engine.clone();
        let timeout = Duration::from_millis(context.root_config.command_timeout);
        std::thread::spawn(move || {
            std::thread::sleep(timeout);
            engine.increment_epoch();
        });
    }

    let instance = match StarshipModule::instantiate(&mut store, &component, &linker) {
        Ok(i) => i,
        Err(e) => {
            log::warn!("Failed to instantiate WASM component: {e}");
            return None;
        }
    };

    let guest = instance.starship_wasm_interface_formatter();
    match guest.call_is_enabled(&mut store) {
        Ok(Ok(true)) => {}
        Ok(Ok(false)) => return None,
        Ok(Err(e)) => {
            log::warn!("WASM is-enabled returned error: {e}");
            return None;
        }
        Err(e) => {
            log::warn!("Failed to call is-enabled: {e}");
            return None;
        }
    }

    let string_formatter = match StringFormatter::new(config.format) {
        Ok(f) => f,
        Err(e) => {
            log::warn!("Error parsing format string in `custom_wasm.format`:\n{e}");
            return None;
        }
    };

    // Meta values must be resolved up front: `map_meta` mappers return string
    // slices, so the owned strings coming out of the component need a home that
    // outlives the formatter.
    let mut meta_values: HashMap<String, String> = HashMap::new();
    for variable in string_formatter.get_variables() {
        match guest.call_map_meta(&mut store, &variable) {
            Ok(Ok(Some(value))) => {
                meta_values.insert(variable, value);
            }
            Ok(Ok(None)) => {}
            Ok(Err(e)) => log::warn!("WASM map-meta error for '{variable}': {e}"),
            Err(e) => {
                log::warn!("Failed to call map-meta for '{variable}': {e}");
                return None;
            }
        }
    }

    // The `map`/`map_style` mappers run on rayon worker threads, and every
    // component call needs `&mut Store`, hence the mutex.
    let store = Mutex::new(store);

    let parsed = string_formatter
        .map_meta(|var, _| {
            meta_values.get(var).map(String::as_str).or(match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
        })
        .map_style(|var| {
            let mut store = store.lock().ok()?;
            match guest.call_map_style(&mut *store, var) {
                Ok(Ok(Some(val))) => Some(Ok(val)),
                Ok(Ok(None)) => match var {
                    "style" => Some(Ok(config.style.to_string())),
                    _ => None,
                },
                Ok(Err(e)) => {
                    log::warn!("WASM map-style error for '{var}': {e}");
                    None
                }
                Err(e) => {
                    log::warn!("Failed to call map-style for '{var}': {e}");
                    None
                }
            }
        })
        .map(|var| {
            let mut store = store.lock().ok()?;
            match guest.call_map(&mut *store, var) {
                Ok(Ok(Some(val))) => {
                    let trimmed = val.trim();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(Ok(trimmed.to_string()))
                    }
                }
                Ok(Ok(None)) => None,
                Ok(Err(e)) => {
                    log::warn!("WASM map error for '{var}': {e}");
                    None
                }
                Err(e) => {
                    log::warn!("Failed to call map for '{var}': {e}");
                    None
                }
            }
        })
        .parse(None, Some(context));

    match parsed {
        Ok(segments) => module.set_segments(segments),
        Err(error) => {
            log::warn!("Error in module `custom_wasm`:\n{error}");
            return None;
        }
    }

    Some(module)
}

/// Return the invoking shell, using `shell` and fallbacking in order to `STARSHIP_SHELL` and "sh"/"cmd"
fn get_shell<'a, 'b>(
    shell_args: &'b [&'a str],
    context: &Context,
) -> (std::borrow::Cow<'a, str>, &'b [&'a str]) {
    if !shell_args.is_empty() {
        (shell_args[0].into(), &shell_args[1..])
    } else if let Some(env_shell) = context.get_env("STARSHIP_SHELL") {
        (env_shell.into(), &[] as &[&str])
    } else if cfg!(windows) {
        // `/C` is added by `handle_shell`
        ("cmd".into(), &[] as &[&str])
    } else {
        ("sh".into(), &[] as &[&str])
    }
}

/// Attempt to run the given command in a shell by passing it as either `stdin` or an argument to `get_shell()`,
/// depending on the configuration or by invoking a platform-specific fallback shell if `shell` is empty.
fn shell_command(cmd: &str, config: &CustomWasmConfig, context: &Context) -> Option<Output> {
    let (shell, shell_args) = get_shell(config.shell.0.as_ref(), context);
    let mut use_stdin = config.use_stdin;

    let mut command = match create_command(shell.as_ref()) {
        Ok(command) => command,
        // Don't attempt to use fallback shell if the user specified a shell
        Err(error) if !shell_args.is_empty() => {
            log::debug!(
                "Error creating command with STARSHIP_SHELL, falling back to fallback shell: {error}"
            );

            // Skip `handle_shell` and just set the shell and command
            use_stdin = Some(!cfg!(windows));

            if cfg!(windows) {
                let mut c = create_command("cmd").ok()?;
                c.arg("/C");
                c
            } else {
                let mut c = create_command("/usr/bin/env").ok()?;
                c.arg("sh");
                c
            }
        }
        _ => return None,
    };

    command
        .current_dir(&context.current_dir)
        .args(shell_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let use_stdin = use_stdin.unwrap_or_else(|| handle_shell(&mut command, &shell, shell_args));

    if !use_stdin {
        command.arg(cmd);
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            log::debug!(
                "Failed to run command with given shell or STARSHIP_SHELL env variable:: {error}"
            );
            return None;
        }
    };

    if use_stdin {
        child.stdin.as_mut()?.write_all(cmd.as_bytes()).ok()?;
    }

    let mut output = child.controlled_with_output();

    if !config.ignore_timeout {
        output = output
            .time_limit(Duration::from_millis(context.root_config.command_timeout))
            .terminate_for_timeout()
    }

    match output.wait().ok()? {
        None => {
            log::warn!("Executing custom command {cmd:?} timed out.");
            log::warn!(
                "You can set command_timeout in your config to a higher value or set ignore_timeout to true for this module to allow longer-running commands to keep executing."
            );
            None
        }
        Some(status) => Some(status),
    }
}

/// Execute the given command capturing all output, and return whether it return 0
fn exec_when(cmd: &str, config: &CustomWasmConfig, context: &Context) -> bool {
    log::trace!("Running '{cmd}'");

    if let Some(output) = shell_command(cmd, config, context) {
        if !output.status.success() {
            log::trace!("non-zero exit code '{:?}'", output.status.code());
            log::trace!(
                "stdout: {}",
                std::str::from_utf8(&output.stdout).unwrap_or("<invalid utf8>")
            );
            log::trace!(
                "stderr: {}",
                std::str::from_utf8(&output.stderr).unwrap_or("<invalid utf8>")
            );
        }

        output.status.success()
    } else {
        log::debug!("Cannot start command");

        false
    }
}

/// If the specified shell refers to `PowerShell`, adds the arguments "-Command -" to the
/// given command.
/// Returns `false` if the shell shell expects scripts as arguments, `true` if as `stdin`.
fn handle_shell(command: &mut Command, shell: &str, shell_args: &[&str]) -> bool {
    let shell_exe = Path::new(shell).file_stem();
    let no_args = shell_args.is_empty();

    match shell_exe.and_then(std::ffi::OsStr::to_str) {
        Some("pwsh" | "powershell") => {
            if no_args {
                command.arg("-NoProfile").arg("-Command").arg("-");
            }
            true
        }
        Some("cmd") => {
            if no_args {
                command.arg("/C");
            }
            false
        }
        Some("nu") => {
            if no_args {
                command.arg("-c");
            }
            false
        }
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;

    #[test]
    fn test_disabled() {
        let actual = ModuleRenderer::new("custom_wasm")
            .config(toml::toml! {
                [custom_wasm]
                disabled = true
                wasm_path = "/tmp/test.wasm"
            })
            .collect();
        assert_eq!(None, actual);
    }

    #[test]
    fn test_missing_wasm_path() {
        let actual = ModuleRenderer::new("custom_wasm")
            .config(toml::toml! {
                [custom_wasm]
                disabled = false
                wasm_path = ""
            })
            .collect();
        assert_eq!(None, actual);
    }

    #[test]
    fn test_nonexistent_wasm_file() {
        let actual = ModuleRenderer::new("custom_wasm")
            .config(toml::toml! {
                [custom_wasm]
                disabled = false
                wasm_path = "/nonexistent/path/module.wasm"
            })
            .collect();
        assert_eq!(None, actual);
    }
}
