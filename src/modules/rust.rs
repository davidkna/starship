use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use serde::Deserialize;
use std::collections::HashMap;

use super::{Context, Module, ModuleConfig};

use crate::configs::rust::RustConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::create_command;
use home::rustup_home;

use std::sync::OnceLock;

use guess_host_triple::guess_host_triple;

type VersionString = String;
type ToolchainString = String;

/// A struct to cache the output of any commands that need to be run.
struct RustToolingEnvironmentInfo {
    /// Rustup settings parsed from $HOME/.rustup/settings.toml
    rustup_settings: OnceLock<RustupSettings>,
    /// Rustc toolchain overrides as contained in the environment or files
    env_toolchain_override: OnceLock<Option<String>>,
    /// The output of `rustup rustc --version` with a fixed toolchain
    rustup_rustc_output: OnceLock<RustupRunRustcVersionOutcome>,
    /// The output of running rustc -vV. Only called if rustup rustc fails or
    /// is unavailable.
    rustc_verbose_output: OnceLock<Option<(VersionString, ToolchainString)>>,
}

impl RustToolingEnvironmentInfo {
    fn new() -> Self {
        Self {
            rustup_settings: OnceLock::new(),
            env_toolchain_override: OnceLock::new(),
            rustup_rustc_output: OnceLock::new(),
            rustc_verbose_output: OnceLock::new(),
        }
    }

    fn get_rustup_settings(&self, context: &Context) -> &RustupSettings {
        self.rustup_settings
            .get_or_init(|| RustupSettings::load(context).unwrap_or_default())
    }

    /// Gets any environmental toolchain overrides without downloading cargo toolchains
    fn get_env_toolchain_override(&self, context: &Context) -> Option<&str> {
        // `$CARGO_HOME/bin/rustc(.exe) --version` may attempt installing a rustup toolchain.
        // https://github.com/starship/starship/issues/417
        //
        // To display appropriate versions preventing `rustc` from downloading toolchains, we have to
        // check
        // 1. `$RUSTUP_TOOLCHAIN`
        // 2. The override list from ~/.rustup/settings.toml (like `rustup override list`)
        // 3. `rust-toolchain` or `rust-toolchain.toml` in `.` or parent directories
        // 4. The `default_toolchain` from ~/.rustup/settings.toml (like `rustup default`)
        // 5. `rustup default` (in addition to the above, this also looks at global fallback config files)
        // as `rustup` does.
        // https://github.com/rust-lang/rustup.rs/tree/eb694fcada7becc5d9d160bf7c623abe84f8971d#override-precedence
        //
        // Probably we have no other way to know whether any toolchain override is specified for the
        // current directory. The following commands also cause toolchain installations.
        // - `rustup show`
        // - `rustup show active-toolchain`
        // - `rustup which`
        self.env_toolchain_override
            .get_or_init(|| {
                let out = env_rustup_toolchain(context)
                    .or_else(|| {
                        self.get_rustup_settings(context)
                            .lookup_override(context.current_dir.as_path())
                    })
                    .or_else(|| find_rust_toolchain_file(context))
                    .or_else(|| {
                        self.get_rustup_settings(context)
                            .default_toolchain()
                            .map(std::string::ToString::to_string)
                    })
                    .or_else(|| execute_rustup_default(context));

                log::debug!("Environmental toolchain override is {out:?}");
                out
            })
            .as_deref()
    }

    /// Gets the output of running `rustup rustc --version` with a toolchain
    /// specified by `self.get_env_toolchain_override()`
    fn get_rustup_rustc_version(&self, context: &Context) -> &RustupRunRustcVersionOutcome {
        self.rustup_rustc_output.get_or_init(|| {
            // Skip the whole rustup path when `rustc` in PATH is not managed by
            // rustup (system-managed rustc).
            if !rustc_is_rustup_managed() {
                log::debug!("rustc is not rustup-managed; skipping rustup version detection");
                return RustupRunRustcVersionOutcome::RustupNotWorking;
            }

            let out = if let Some(toolchain) = self.get_env_toolchain_override(context) {
                // Try reading the version from the toolchain's on-disk files
                // first — no subprocess needed.
                let host_triple = match self.get_rustup_settings(context).default_host_triple() {
                    Some(triple) => Some(triple),
                    None => guess_host_triple(),
                };
                find_toolchain_dir(toolchain, host_triple)
                    .and_then(|dir| get_version_from_toolchain_dir(&dir))
                    .map(RustupRunRustcVersionOutcome::RustcVersion)
                    .unwrap_or_else(|| run_rustc_from_toolchain(toolchain, context))
            } else {
                RustupRunRustcVersionOutcome::ToolchainUnknown
            };

            log::debug!("Rustup rustc version is {out:?}");
            out
        })
    }

    /// Gets the (version, toolchain) string as returned by `rustc -vV`
    fn get_rustc_verbose_version(&self, context: &Context) -> Option<(&str, &str)> {
        let toolchain = self.get_rustup_settings(context).default_toolchain();

        self.rustc_verbose_output
            .get_or_init(|| {
                let Output { status, stdout, .. } = create_command("rustc")
                    .and_then(|mut cmd| {
                        cmd.args(["-Vv"]).current_dir(&context.current_dir).output()
                    })
                    .ok()?;
                if !status.success() {
                    return None;
                }
                let out =
                    format_rustc_version_verbose(std::str::from_utf8(&stdout).ok()?, toolchain);

                log::debug!("Rustup verbose version is {out:?}");
                out
            })
            .as_ref()
            .map(|(x, y)| (&x[..], &y[..]))
    }
}

/// Creates a module with the current Rust version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("rust");
    let config = RustConfig::try_load(module.config);

    let is_rs_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_rs_project {
        return None;
    }

    let rust_env_info = RustToolingEnvironmentInfo::new();

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => get_module_version(context, &config, &rust_env_info).map(Ok),
                "numver" => get_module_numeric_version(context, &config, &rust_env_info).map(Ok),
                "toolchain" => get_toolchain_version(context, &config, &rust_env_info).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `rust`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_module_version(
    context: &Context,
    config: &RustConfig,
    rust_env_info: &RustToolingEnvironmentInfo,
) -> Option<String> {
    type Outcome = RustupRunRustcVersionOutcome;

    match rust_env_info.get_rustup_rustc_version(context) {
        Outcome::RustcVersion(rustc_version) => {
            format_rustc_version(rustc_version, config.version_format)
        }
        Outcome::RustupNotWorking | Outcome::ToolchainUnknown => {
            // If `rustc` is not managed by rustup, `rustup` can't be executed, or there
            // is no environmental toolchain, we can execute `rustc --version` without
            // triggering a toolchain download
            format_rustc_version(&execute_rustc_version(context)?, config.version_format)
        }
        Outcome::ToolchainNotInstalled(name) => Some(name.to_string()),
        Outcome::Err => None,
    }
}

fn get_module_numeric_version(
    context: &Context,
    _config: &RustConfig,
    rust_env_info: &RustToolingEnvironmentInfo,
) -> Option<String> {
    type Outcome = RustupRunRustcVersionOutcome;

    match rust_env_info.get_rustup_rustc_version(context) {
        Outcome::RustcVersion(version) => {
            let release = version.split_whitespace().nth(1).unwrap_or(version);
            Some(format_semver(release))
        }
        Outcome::RustupNotWorking | Outcome::ToolchainUnknown => {
            let (numver, _toolchain) = rust_env_info.get_rustc_verbose_version(context)?;
            Some(numver.to_string())
        }
        Outcome::ToolchainNotInstalled(_) | RustupRunRustcVersionOutcome::Err => None,
    }
}

fn get_toolchain_version(
    context: &Context,
    _config: &RustConfig,
    rust_env_info: &RustToolingEnvironmentInfo,
) -> Option<String> {
    type Outcome = RustupRunRustcVersionOutcome;

    let settings_host_triple = rust_env_info
        .get_rustup_settings(context)
        .default_host_triple();
    let default_host_triple = if settings_host_triple.is_none() {
        guess_host_triple()
    } else {
        settings_host_triple
    };

    match rust_env_info.get_rustup_rustc_version(context) {
        Outcome::RustcVersion(_) | Outcome::ToolchainNotInstalled(_) => {
            let toolchain_override = rust_env_info
                .get_env_toolchain_override(context)
                // This match arm should only trigger if the toolchain override
                // is not None because of how get_rustup_rustc_version works
                .expect("Toolchain override was None: programming error.");
            Some(format_toolchain(toolchain_override, default_host_triple))
        }
        Outcome::RustupNotWorking | Outcome::ToolchainUnknown => {
            let (_numver, toolchain) = rust_env_info.get_rustc_verbose_version(context)?;
            Some(format_toolchain(toolchain, default_host_triple))
        }
        Outcome::Err => None,
    }
}

fn env_rustup_toolchain(context: &Context) -> Option<String> {
    log::trace!("Searching for rustup toolchain in environment.");
    let val = context.get_env("RUSTUP_TOOLCHAIN")?;
    Some(val.trim().to_owned())
}

fn execute_rustup_default(context: &Context) -> Option<String> {
    log::trace!("Searching for toolchain with rustup default");
    // `rustup default` output is:
    //    stable-x86_64-apple-darwin (default)
    context
        .exec_cmd("rustup", &["default"])?
        .stdout
        .split_whitespace()
        .next()
        .map(str::to_owned)
}

fn find_rust_toolchain_file(context: &Context) -> Option<String> {
    log::trace!("Searching for toolchain in toolchain file");
    // Look for 'rust-toolchain' or 'rust-toolchain.toml' as rustup does.
    // for more information:
    // https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
    // for the implementation in 'rustup':
    // https://github.com/rust-lang/rustup/blob/a45e4cd21748b04472fce51ba29999ee4b62bdec/src/config.rs#L631

    #[derive(Deserialize)]
    struct OverrideFile {
        toolchain: ToolchainSection,
    }

    #[derive(Deserialize)]
    struct ToolchainSection {
        channel: Option<String>,
    }

    fn read_channel(path: &Path, only_toml: bool) -> Option<String> {
        let contents = fs::read_to_string(path).ok()?;

        match contents.lines().count() {
            0 => None,
            1 if !only_toml => Some(contents),
            _ => {
                toml::from_str::<OverrideFile>(&contents)
                    .ok()?
                    .toolchain
                    .channel
            }
        }
        .filter(|c| !c.trim().is_empty())
        .map(|c| c.trim().to_owned())
        .filter(|c| {
            // A toolchain name must either contain no path separators (e.g.
            // "stable", "nightly", "1.34.0") or be an absolute path (a custom
            // toolchain directory).  Relative paths are rejected because rustup
            // itself does not accept them.
            let p = Path::new(c.as_str());
            let valid = p.components().count() <= 1 || p.is_absolute();
            if !valid {
                log::warn!(
                    "Ignoring toolchain '{c}' from {path:?}: relative paths are not permitted"
                );
            }
            valid
        })
    }

    if context
        .dir_contents()
        .is_ok_and(|dir| dir.has_file("rust-toolchain"))
        && let Some(toolchain) = read_channel(&context.current_dir.join("rust-toolchain"), false)
    {
        return Some(toolchain);
    }

    if context
        .dir_contents()
        .is_ok_and(|dir| dir.has_file("rust-toolchain.toml"))
        && let Some(toolchain) =
            read_channel(&context.current_dir.join("rust-toolchain.toml"), true)
    {
        return Some(toolchain);
    }

    let mut dir = &*context.current_dir;
    loop {
        if let Some(toolchain) = read_channel(&dir.join("rust-toolchain"), false) {
            return Some(toolchain);
        }
        if let Some(toolchain) = read_channel(&dir.join("rust-toolchain.toml"), true) {
            return Some(toolchain);
        }
        dir = dir.parent()?;
    }
}

fn extract_toolchain_from_rustup_run_rustc_version(output: Output) -> RustupRunRustcVersionOutcome {
    if output.status.success() {
        if let Ok(output) = String::from_utf8(output.stdout) {
            return RustupRunRustcVersionOutcome::RustcVersion(output);
        }
    } else if let Ok(stderr) = String::from_utf8(output.stderr)
        && stderr.starts_with("error: toolchain '")
        && stderr.ends_with("' is not installed\n")
    {
        let stderr = stderr
            ["error: toolchain '".len()..stderr.len() - "' is not installed\n".len()]
            .to_owned();
        return RustupRunRustcVersionOutcome::ToolchainNotInstalled(stderr);
    }
    RustupRunRustcVersionOutcome::Err
}

fn execute_rustc_version(context: &Context) -> Option<String> {
    context
        .exec_cmd("rustc", &["--version"])
        .map(|o| o.stdout)
        .filter(|s| !s.is_empty())
}

/// Returns `true` when the `rustc` binary in PATH is managed by rustup —
/// i.e. it lives either under `$CARGO_HOME/bin` (the rustup proxy shim) or
/// directly inside a rustup toolchain directory.
fn rustc_is_rustup_managed() -> bool {
    let Ok(rustc_path) = which::which("rustc") else {
        return false;
    };
    // Resolve symlinks: package managers may link the rustup proxies into a
    // bin directory outside CARGO_HOME (e.g. Homebrew's rustup formula).
    let rustc_path = rustc_path.canonicalize().unwrap_or(rustc_path);
    let under_rustup = rustup_home()
        .map(|h| rustc_path.starts_with(h.join("toolchains")))
        .unwrap_or(false);
    let under_cargo = home::cargo_home()
        .map(|h| rustc_path.starts_with(h.join("bin")))
        .unwrap_or(false);
    // A rustup proxy `rustc` always sits next to a `rustup` binary. A distro
    // rustc that happens to share a bin directory with a distro rustup is a
    // false positive, but that only costs the (safe) rustup detection path,
    // while a missed proxy risks triggering a toolchain download (#417).
    let next_to_rustup = rustc_path.parent().is_some_and(|dir| {
        dir.join(format!("rustup{}", std::env::consts::EXE_SUFFIX))
            .is_file()
    });
    log::trace!(
        "rustc at {rustc_path:?}: under_rustup={under_rustup}, under_cargo={under_cargo}, next_to_rustup={next_to_rustup}"
    );
    under_rustup || under_cargo || next_to_rustup
}

/// Falls back to running `rustc --version` directly from the toolchain directory,
/// or via `rustup run <toolchain> rustc --version` if the toolchain directory is
/// not found.
fn run_rustc_from_toolchain(toolchain: &str, context: &Context) -> RustupRunRustcVersionOutcome {
    rustup_home()
        .map(|rustup_folder| {
            rustup_folder
                .join("toolchains")
                .join(toolchain)
                .join("bin")
                .join("rustc")
        })
        .and_then(|rustc| {
            log::trace!("Running rustc --version directly with {rustc:?}");
            create_command(rustc).map(|mut cmd| {
                cmd.arg("--version");
                cmd
            })
        })
        .or_else(|_| {
            // Toolchain name may be short ("stable"/"nightly") rather than fully-qualified.
            log::trace!("Running rustup {toolchain} rustc --version");
            create_command("rustup").map(|mut cmd| {
                cmd.args(["run", toolchain, "rustc", "--version"]);
                cmd
            })
        })
        .and_then(|mut cmd| cmd.current_dir(&context.current_dir).output())
        .map(extract_toolchain_from_rustup_run_rustc_version)
        .unwrap_or(RustupRunRustcVersionOutcome::RustupNotWorking)
}

/// Resolves a toolchain name to its directory under `~/.rustup/toolchains`.
///
/// The name may be fully qualified (`stable-aarch64-apple-darwin`) or a short
/// channel name (`stable`); short names are completed with the default host
/// triple, or failing that the first matching directory.
///
/// Path-based toolchains (custom toolchain directories) return `None`: they
/// are not installed under `~/.rustup/toolchains` and their on-disk metadata
/// may not match the actual compiler, so the caller should fall through to
/// running rustc/rustup instead.
fn find_toolchain_dir(toolchain: &str, host_triple: Option<&str>) -> Option<PathBuf> {
    if Path::new(toolchain).components().count() > 1 {
        return None;
    }

    let toolchains_dir = rustup_home().ok()?.join("toolchains");

    let exact = toolchains_dir.join(toolchain);
    if exact.is_dir() {
        return Some(exact);
    }

    if let Some(triple) = host_triple {
        let qualified = toolchains_dir.join(format!("{toolchain}-{triple}"));
        if qualified.is_dir() {
            return Some(qualified);
        }
    }

    let prefix = format!("{toolchain}-");
    fs::read_dir(&toolchains_dir)
        .ok()?
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy().starts_with(&prefix))
        .map(|e| e.path())
}

/// Reads the Rust version from an installed toolchain's on-disk files without
/// spawning a subprocess: first from the `rustc(1)` man page header (a few KiB;
/// recent toolchains embed the full version there), then from the channel
/// manifest (`lib/rustlib/multirust-channel-manifest.toml`, around 1 MiB).
/// Returns a string in `rustc --version` style, e.g.
/// `"rustc 1.77.0 (aeda7d245 2024-03-13)"`.
fn get_version_from_toolchain_dir(toolchain_dir: &Path) -> Option<String> {
    let man_path = toolchain_dir
        .join("share")
        .join("man")
        .join("man1")
        .join("rustc.1");
    if let Ok(man) = fs::read_to_string(man_path)
        && let Some(version) = scan_man_page_rust_version(&man)
    {
        return Some(format!("rustc {version}"));
    }

    let manifest_path = toolchain_dir
        .join("lib")
        .join("rustlib")
        .join("multirust-channel-manifest.toml");
    let content = fs::read_to_string(manifest_path).ok()?;
    let version = scan_manifest_rust_version(&content)?;
    Some(format!("rustc {version}"))
}

/// Extracts the version from the `.TH` header of a toolchain's `rustc(1)` man
/// page, e.g.
/// `.TH RUSTC "1" "April 2019" "rustc 1.77.0 (aeda7d245 2024-03-13)" "User Commands"`.
///
/// Only the full verbose form (`<version> (<hash> <date>)`) is accepted; older
/// toolchains shipped an `<INSERT VERSION HERE>` placeholder or a bare version
/// without the hash — returns `None` for those so the caller falls through to
/// the channel manifest.
fn scan_man_page_rust_version(content: &str) -> Option<&str> {
    let line = content.lines().find(|l| l.starts_with(".TH RUSTC"))?;
    let rest = &line[line.find("\"rustc ")? + "\"rustc ".len()..];
    let version = &rest[..rest.find('"')?];
    (version.starts_with(|c: char| c.is_ascii_digit())
        && version.contains(" (")
        && version.ends_with(')'))
    .then_some(version)
}

/// Extracts the `[pkg.rust]` version string from a channel manifest without
/// parsing the full TOML.  Returns the bare version, e.g.
/// `"1.77.0 (aeda7d245 2024-03-13)"`.
fn scan_manifest_rust_version(content: &str) -> Option<&str> {
    let mut in_pkg_rust = false;
    for line in content.lines() {
        if line == "[pkg.rust]" {
            in_pkg_rust = true;
            continue;
        }
        if in_pkg_rust {
            if line.starts_with('[') {
                return None; // left the section without finding version
            }
            if let Some(rest) = line.strip_prefix("version = \"") {
                return rest.strip_suffix('"');
            }
        }
    }
    None
}

fn format_rustc_version(rustc_version: &str, version_format: &str) -> Option<String> {
    let version = rustc_version
        // split into ["rustc", "1.34.0", ...]
        .split_whitespace()
        // get down to "1.34.0"
        .nth(1)?;

    match VersionFormatter::format_version(version, version_format) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            log::warn!("Error formatting `rust` version:\n{error}");
            Some(format!("v{version}"))
        }
    }
}

fn format_toolchain(toolchain: &str, default_host_triple: Option<&str>) -> String {
    default_host_triple
        .map_or(toolchain, |triple| {
            toolchain.trim_end_matches(&format!("-{triple}"))
        })
        .to_owned()
}

fn format_rustc_version_verbose(stdout: &str, toolchain: Option<&str>) -> Option<(String, String)> {
    let (mut release, mut host) = (None, None);
    for line in stdout.lines() {
        if line.starts_with("release: ") {
            release = Some(line.trim_start_matches("release: "));
        }
        if line.starts_with("host: ") {
            host = Some(line.trim_start_matches("host: "));
        }
    }
    let (release, host) = (release?, host?);
    let version = format_semver(release);
    let toolchain = toolchain.map_or_else(|| host.to_string(), ToOwned::to_owned);
    Some((version, toolchain))
}

fn format_semver(semver: &str) -> String {
    format!("v{}", semver.find('-').map_or(semver, |i| &semver[..i]))
}

#[derive(Debug, PartialEq)]
enum RustupRunRustcVersionOutcome {
    RustcVersion(String),
    ToolchainNotInstalled(String),
    ToolchainUnknown,
    RustupNotWorking,
    Err,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct RustupSettings {
    default_host_triple: Option<String>,
    default_toolchain: Option<String>,
    overrides: HashMap<PathBuf, String>,
    version: Option<String>,
}

#[inline]
#[cfg(windows)]
fn strip_dos_path(path: PathBuf) -> PathBuf {
    // Use the display version of the path to strip \\?\
    let path = path.to_string_lossy();
    PathBuf::from(path.strip_prefix(r"\\?\").unwrap_or(&path))
}

#[inline]
#[cfg(not(windows))]
fn strip_dos_path(path: PathBuf) -> PathBuf {
    path
}

impl RustupSettings {
    fn load(_context: &Context) -> Option<Self> {
        let path = rustup_home().ok()?.join("settings.toml");
        Self::from_toml_str(&fs::read_to_string(path).ok()?)
    }

    fn from_toml_str(toml_str: &str) -> Option<Self> {
        let settings = toml::from_str::<Self>(toml_str).ok()?;
        if settings.version.as_deref() == Some("12") {
            Some(settings)
        } else {
            log::warn!(
                r#"Rustup settings version is {:?}, expected "12""#,
                settings.version
            );
            None
        }
    }

    fn default_host_triple(&self) -> Option<&str> {
        self.default_host_triple.as_deref()
    }

    fn default_toolchain(&self) -> Option<&str> {
        self.default_toolchain.as_deref()
    }

    fn lookup_override(&self, cwd: &Path) -> Option<String> {
        let cwd = strip_dos_path(cwd.to_owned());
        self.overrides
            .iter()
            .map(|(dir, toolchain)| (strip_dos_path(dir.clone()), toolchain))
            .filter(|(dir, _)| cwd.starts_with(dir))
            .max_by_key(|(dir, _)| dir.components().count())
            .map(|(_, name)| name.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::context::{Env, Properties, Shell, Target};
    use std::io;
    use std::process::{ExitStatus, Output};
    use std::sync::LazyLock;

    use super::*;

    #[test]
    fn test_rustup_settings_from_toml_value() {
        assert_eq!(
            RustupSettings::from_toml_str(
                r#"
default_host_triple = "x86_64-unknown-linux-gnu"
default_toolchain = "stable"
version = "12"

[overrides]
"/home/user/src/starship" = "1.40.0-x86_64-unknown-linux-gnu"
"#
            ),
            Some(RustupSettings {
                default_host_triple: Some("x86_64-unknown-linux-gnu".to_owned()),
                default_toolchain: Some("stable".to_owned()),
                overrides: vec![(
                    "/home/user/src/starship".into(),
                    "1.40.0-x86_64-unknown-linux-gnu".to_owned(),
                )]
                .into_iter()
                .collect(),
                version: Some("12".to_string())
            }),
        );

        // Invalid or missing version key causes a failure
        assert_eq!(
            RustupSettings::from_toml_str(
                r#"
                default_host_triple = "x86_64-unknown-linux-gnu"
                default_toolchain = "stable"

                [overrides]
                "/home/user/src/starship" = "1.39.0-x86_64-unknown-linux-gnu"
            "#
            ),
            None,
        );
    }

    #[test]
    fn test_override_matches_correct_directories() {
        let test_settings = RustupSettings::from_toml_str(
            r#"
default_host_triple = "x86_64-unknown-linux-gnu"
default_toolchain = "stable"
version = "12"

[overrides]
"/home/user/src/a" = "beta-x86_64-unknown-linux-gnu"
"/home/user/src/b" = "nightly-x86_64-unknown-linux-gnu"
"/home/user/src/b/d c" = "stable-x86_64-pc-windows-msvc"
"#,
        )
        .unwrap();

        static OVERRIDES_CWD_A: &str = "/home/user/src/a/src";
        static OVERRIDES_CWD_B: &str = "/home/user/src/b/tests";
        static OVERRIDES_CWD_C: &str = "/home/user/src/c/examples";
        static OVERRIDES_CWD_D: &str = "/home/user/src/b/d c/spaces";
        static OVERRIDES_CWD_E: &str = "/home/user/src/b_and_more";
        static OVERRIDES_CWD_F: &str = "/home/user/src/b";

        static BETA_TOOLCHAIN: &str = "beta-x86_64-unknown-linux-gnu";
        static NIGHTLY_TOOLCHAIN: &str = "nightly-x86_64-unknown-linux-gnu";
        static STABLE_TOOLCHAIN: &str = "stable-x86_64-pc-windows-msvc";

        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_A.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_B.as_ref()),
            Some(NIGHTLY_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_C.as_ref()),
            None
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_D.as_ref()),
            Some(STABLE_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_E.as_ref()),
            None
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_F.as_ref()),
            Some(NIGHTLY_TOOLCHAIN.to_string())
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_extract_toolchain_from_override_with_dospath() {
        let test_settings = RustupSettings::from_toml_str(
            r#"
default_host_triple = "x86_64-unknown-linux-gnu"
default_toolchain = "stable"
version = "12"

[overrides]
"C:\\src1" = "beta-x86_64-unknown-linux-gnu"
"\\\\?\\C:\\src2" = "beta-x86_64-unknown-linux-gnu"
"#,
        )
        .unwrap();
        static OVERRIDES_CWD_A: &str = r"\\?\C:\src1";
        static OVERRIDES_CWD_B: &str = r"C:\src1";
        static OVERRIDES_CWD_C: &str = r"\\?\C:\src2";
        static OVERRIDES_CWD_D: &str = r"C:\src2";

        static BETA_TOOLCHAIN: &str = "beta-x86_64-unknown-linux-gnu";

        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_A.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_B.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_C.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
        assert_eq!(
            test_settings.lookup_override(OVERRIDES_CWD_D.as_ref()),
            Some(BETA_TOOLCHAIN.to_string())
        );
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn test_extract_toolchain_from_rustup_run_rustc_version() {
        #[cfg(unix)]
        use std::os::unix::process::ExitStatusExt as _;
        #[cfg(windows)]
        use std::os::windows::process::ExitStatusExt as _;

        static RUSTC_VERSION: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"rustc 1.34.0\n"[..].to_owned(),
            stderr: vec![],
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(RUSTC_VERSION.clone()),
            RustupRunRustcVersionOutcome::RustcVersion("rustc 1.34.0\n".to_owned()),
        );

        static TOOLCHAIN_NAME: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"error: toolchain 'channel-triple' is not installed\n"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(TOOLCHAIN_NAME.clone()),
            RustupRunRustcVersionOutcome::ToolchainNotInstalled("channel-triple".to_owned()),
        );

        static INVALID_STDOUT: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(0),
            stdout: b"\xc3\x28"[..].to_owned(),
            stderr: vec![],
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(INVALID_STDOUT.clone()),
            RustupRunRustcVersionOutcome::Err,
        );

        static INVALID_STDERR: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"\xc3\x28"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(INVALID_STDERR.clone()),
            RustupRunRustcVersionOutcome::Err,
        );

        static UNEXPECTED_FORMAT_OF_ERROR: LazyLock<Output> = LazyLock::new(|| Output {
            status: ExitStatus::from_raw(1),
            stdout: vec![],
            stderr: b"error:"[..].to_owned(),
        });
        assert_eq!(
            extract_toolchain_from_rustup_run_rustc_version(UNEXPECTED_FORMAT_OF_ERROR.clone()),
            RustupRunRustcVersionOutcome::Err,
        );
    }

    #[test]
    fn test_format_rustc_version() {
        let config = RustConfig::default();
        let rustc_stable = "rustc 1.34.0 (91856ed52 2019-04-10)";
        let rustc_beta = "rustc 1.34.0-beta.1 (2bc1d406d 2019-04-10)";
        let rustc_nightly = "rustc 1.34.0-nightly (b139669f3 2019-04-10)";
        assert_eq!(
            format_rustc_version(rustc_nightly, config.version_format),
            Some("v1.34.0-nightly".to_string())
        );
        assert_eq!(
            format_rustc_version(rustc_beta, config.version_format),
            Some("v1.34.0-beta.1".to_string())
        );
        assert_eq!(
            format_rustc_version(rustc_stable, config.version_format),
            Some("v1.34.0".to_string())
        );
        assert_eq!(
            format_rustc_version("rustc 1.34.0", config.version_format),
            Some("v1.34.0".to_string())
        );
    }

    #[test]
    fn test_find_rust_toolchain_file() -> io::Result<()> {
        // `rust-toolchain` with toolchain in one line
        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("rust-toolchain"), "1.34.0")?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain` in toml format
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("rust-toolchain"),
            "[toolchain]\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain` in toml format with new lines
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("rust-toolchain"),
            "\n\n[toolchain]\n\n\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain` in parent directory.
        let dir = tempfile::tempdir()?;
        let child_dir_path = dir.path().join("child");
        fs::create_dir(&child_dir_path)?;
        fs::write(
            dir.path().join("rust-toolchain"),
            "\n\n[toolchain]\n\n\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            child_dir_path.clone(),
            child_dir_path,
            Env::default(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain.toml` with toolchain in one line
        // This should not work!
        // See https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("rust-toolchain.toml"), "1.34.0")?;

        let context = Context::new_with_shell_and_path(
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );

        assert_eq!(find_rust_toolchain_file(&context), None);
        dir.close()?;

        // `rust-toolchain.toml` in toml format
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain.toml` in toml format with new lines
        let dir = tempfile::tempdir()?;
        fs::write(
            dir.path().join("rust-toolchain.toml"),
            "\n\n[toolchain]\n\n\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()?;

        // `rust-toolchain.toml` in parent directory.
        let dir = tempfile::tempdir()?;
        let child_dir_path = dir.path().join("child");
        fs::create_dir(&child_dir_path)?;
        fs::write(
            dir.path().join("rust-toolchain.toml"),
            "\n\n[toolchain]\n\n\nchannel = \"1.34.0\"",
        )?;

        let context = Context::new_with_shell_and_path(
            Properties::default(),
            Shell::Unknown,
            Target::Main,
            child_dir_path.clone(),
            child_dir_path,
            Env::default(),
        );

        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("1.34.0".to_owned())
        );
        dir.close()
    }

    #[test]
    fn test_format_rustc_version_verbose() {
        macro_rules! test {
            () => {};
            (($input:expr, $toolchain:expr) => $expected:expr $(,$($rest:tt)*)?) => {
                assert_eq!(
                    format_rustc_version_verbose($input, $toolchain)
                        .as_ref()
                        .map(|(s1, s2)| (&**s1, &**s2)),
                    $expected,
                );
                test!($($($rest)*)?);
            };
        }

        static STABLE: &str = r"rustc 1.40.0 (73528e339 2019-12-16)
binary: rustc
commit-hash: 73528e339aae0f17a15ffa49a8ac608f50c6cf14
commit-date: 2019-12-16
host: x86_64-unknown-linux-gnu
release: 1.40.0
LLVM version: 9.0
";

        static BETA: &str = r"rustc 1.41.0-beta.1 (eb3f7c2d3 2019-12-17)
binary: rustc
commit-hash: eb3f7c2d3aec576f47eba854cfbd3c1187b8a2a0
commit-date: 2019-12-17
host: x86_64-unknown-linux-gnu
release: 1.41.0-beta.1
LLVM version: 9.0
";

        static NIGHTLY: &str = r"rustc 1.42.0-nightly (da3629b05 2019-12-29)
binary: rustc
commit-hash: da3629b05f8f1b425a738bfe9fe9aedd47c5417a
commit-date: 2019-12-29
host: x86_64-unknown-linux-gnu
release: 1.42.0-nightly
LLVM version: 9.0
";

        test!(
            (STABLE, None) => Some(("v1.40.0", "x86_64-unknown-linux-gnu")),
            (STABLE, Some("stable")) => Some(("v1.40.0", "stable")),
            (BETA, None) => Some(("v1.41.0", "x86_64-unknown-linux-gnu")),
            (BETA, Some("beta")) => Some(("v1.41.0", "beta")),
            (NIGHTLY, None) => Some(("v1.42.0", "x86_64-unknown-linux-gnu")),
            (NIGHTLY, Some("nightly")) => Some(("v1.42.0", "nightly")),
            ("", None) => None,
            ("", Some("stable")) => None,
        );
    }

    #[test]
    fn test_find_rust_toolchain_file_rejects_relative_paths() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        // Relative path with separator — must be rejected
        fs::write(dir.path().join("rust-toolchain"), "../some-toolchain")?;
        let context = Context::new_with_shell_and_path(
            Default::default(),
            Shell::Unknown,
            Target::Main,
            dir.path().into(),
            dir.path().into(),
            Env::default(),
        );
        assert_eq!(find_rust_toolchain_file(&context), None);

        // Absolute path — must be accepted
        fs::write(dir.path().join("rust-toolchain"), "/opt/my-toolchain")?;
        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("/opt/my-toolchain".to_owned()),
        );

        // Plain channel name (no separator) — must be accepted
        fs::write(dir.path().join("rust-toolchain"), "stable")?;
        assert_eq!(
            find_rust_toolchain_file(&context),
            Some("stable".to_owned()),
        );

        dir.close()
    }

    #[test]
    fn test_scan_manifest_rust_version_stable() {
        let manifest = r#"manifest-version = "2"
date = "2026-03-26"

[pkg.rust]
version = "1.94.1 (e408947bf 2026-03-25)"
target = {}
"#;
        assert_eq!(
            scan_manifest_rust_version(manifest),
            Some("1.94.1 (e408947bf 2026-03-25)"),
            "should extract version from [pkg.rust] section",
        );
    }

    #[test]
    fn test_scan_manifest_rust_version_ignores_other_sections() {
        let manifest = r#"[pkg.other]
version = "0.0.1"

[pkg.rust]
version = "1.94.1 (e408947bf 2026-03-25)"
"#;
        assert_eq!(
            scan_manifest_rust_version(manifest),
            Some("1.94.1 (e408947bf 2026-03-25)"),
            "should not match version = in sections before [pkg.rust]",
        );
    }

    #[test]
    fn test_scan_manifest_rust_version_missing_section() {
        let manifest = "[pkg.other]\nversion = \"1.0\"\n";
        assert_eq!(
            scan_manifest_rust_version(manifest),
            None,
            "should return None when [pkg.rust] section is absent",
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_find_toolchain_dir_rejects_path_based_toolchains() {
        assert_eq!(find_toolchain_dir("/opt/my-toolchain", None), None);
        assert_eq!(find_toolchain_dir("../my-toolchain", None), None);
    }

    #[test]
    fn test_scan_man_page_rust_version() {
        let man = r#".TH RUSTC "1" "April 2019" "rustc 1.97.0-beta.6 (b2282dd56 2026-07-01)" "User Commands"
.SH NAME
rustc \- The Rust compiler
"#;
        assert_eq!(
            scan_man_page_rust_version(man),
            Some("1.97.0-beta.6 (b2282dd56 2026-07-01)"),
            "should extract the version from the .TH header",
        );
    }

    #[test]
    fn test_scan_man_page_rust_version_placeholder() {
        let man = r#".TH RUSTC "1" "April 2019" "rustc <INSERT VERSION HERE>" "User Commands"
.SH NAME
rustc \- The Rust compiler
"#;
        assert_eq!(
            scan_man_page_rust_version(man),
            None,
            "should reject the placeholder shipped by older toolchains",
        );
    }

    #[test]
    fn test_scan_man_page_rust_version_bare_version() {
        let man = r#".TH RUSTC "1" "April 2019" "rustc 1.20.0" "User Commands"
.SH NAME
rustc \- The Rust compiler
"#;
        assert_eq!(
            scan_man_page_rust_version(man),
            None,
            "should reject a bare version without hash and date",
        );
    }

    #[test]
    fn test_scan_manifest_rust_version_missing_version_key() {
        let manifest = "[pkg.rust]\ntarget = {}\n";
        assert_eq!(
            scan_manifest_rust_version(manifest),
            None,
            "should return None when [pkg.rust] has no version key",
        );
    }
}
