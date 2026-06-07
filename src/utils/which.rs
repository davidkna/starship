use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// A wrapper around `which::which` that resolves Scoop shim executables on Windows
/// to their actual target executables directly.
pub fn which<T: AsRef<OsStr>>(binary_name: T) -> Result<PathBuf, which::Error> {
    let path = which::which(binary_name)?;
    #[cfg(windows)]
    {
        Ok(resolve_scoop_shim(path))
    }
    #[cfg(not(windows))]
    {
        Ok(path)
    }
}

/// Helper function to resolve Scoop shim executables
pub fn resolve_scoop_shim(path: PathBuf) -> PathBuf {
    if !cfg!(windows) {
        return path;
    }
    if is_in_scoop_shims_dir(&path) {
        let mut shim_path = path.clone();
        shim_path.set_extension("shim");
        if let Some(target) = parse_shim_file(&shim_path)
            && let Some(parent) = path.parent()
        {
            let resolved = parent.join(target);
            if resolved.exists() {
                return resolved;
            }
        }
    }
    path
}

/// Retrieve the possible Scoop shims directories on the system.
fn get_scoop_shims_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    // Per-user Scoop root
    let per_user_root = env::var("SCOOP")
        .filter(str::is_empty)
        .map(PathBuf::from)
        .or_else(|| {
        env::var("USERPROFILE").map(|up| PathBuf::from(up).join("scoop"))
        .map(Path::)

    if let Some(root) = per_user_root {
        dirs.push(root.join("shims"));
    }

    // Global Scoop root
    let global_root = if let Ok(scoop_global_env) = env::var("SCOOP_GLOBAL") {
        if !scoop_global_env.is_empty() {
            Some(PathBuf::from(scoop_global_env))
        } else {
            None
        }
    } else {
        None
    };

    let global_root = global_root.or_else(|| {
        env::var("ProgramData")
            .ok()
            .map(|pd| PathBuf::from(pd).join("scoop"))
    });

    if let Some(root) = global_root {
        dirs.push(root.join("shims"));
    }

    dirs
}

/// Check if a path resides inside one of the Scoop shims directories.
fn is_in_scoop_shims_dir(path: &Path) -> bool {
    let parent = match path.parent() {
        Some(p) => p,
        None => return false,
    };

    let canonical_parent = match dunce::canonicalize(parent) {
        Ok(p) => p,
        Err(_) => return false,
    };

    for shims_dir in get_scoop_shims_dirs() {
        if let Ok(canonical_shims_dir) = dunce::canonicalize(&shims_dir) {
            if canonical_parent == canonical_shims_dir {
                return true;
            }
        }
    }
    false
}

/// Parse a Scoop `.shim` file and return the target path if no arguments are specified.
fn parse_shim_file(shim_path: &Path) -> Option<PathBuf> {
    let content = std::fs::read_to_string(shim_path).ok()?;
    let mut target_path = None;
    let mut has_args = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim();
            let val = val.trim().trim_matches(|c| c == '"' || c == '\'').trim();
            if key.eq_ignore_ascii_case("path") {
                target_path = Some(PathBuf::from(val));
            } else if key.eq_ignore_ascii_case("args") && !val.is_empty() {
                has_args = true;
            }
        }
    }

    if has_args { None } else { target_path }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_parse_shim_file_simple_path() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        let mut file = File::create(&shim_path).unwrap();
        writeln!(file, "path = C:\\scoop\\apps\\git\\current\\bin\\git.exe").unwrap();
        let parsed = parse_shim_file(&shim_path);
        assert_eq!(
            parsed,
            Some(PathBuf::from("C:\\scoop\\apps\\git\\current\\bin\\git.exe"))
        );
    }

    #[test]
    fn test_parse_shim_file_double_quotes() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        let mut file = File::create(&shim_path).unwrap();
        writeln!(
            file,
            "path = \"C:\\scoop\\apps\\git\\current\\bin\\git.exe\""
        )
        .unwrap();
        let parsed = parse_shim_file(&shim_path);
        assert_eq!(
            parsed,
            Some(PathBuf::from("C:\\scoop\\apps\\git\\current\\bin\\git.exe"))
        );
    }

    #[test]
    fn test_parse_shim_file_single_quotes() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        let mut file = File::create(&shim_path).unwrap();
        writeln!(file, "path = 'C:\\scoop\\apps\\git\\current\\bin\\git.exe'").unwrap();
        let parsed = parse_shim_file(&shim_path);
        assert_eq!(
            parsed,
            Some(PathBuf::from("C:\\scoop\\apps\\git\\current\\bin\\git.exe"))
        );
    }

    #[test]
    fn test_parse_shim_file_non_empty_args() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        let mut file = File::create(&shim_path).unwrap();
        writeln!(file, "path = C:\\scoop\\apps\\git\\current\\bin\\git.exe").unwrap();
        writeln!(file, "args = --version").unwrap();
        let parsed = parse_shim_file(&shim_path);
        assert_eq!(parsed, None);
    }

    #[test]
    fn test_parse_shim_file_empty_args() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        let mut file = File::create(&shim_path).unwrap();
        writeln!(file, "path = C:\\scoop\\apps\\git\\current\\bin\\git.exe").unwrap();
        writeln!(file, "args = ").unwrap();
        let parsed = parse_shim_file(&shim_path);
        assert_eq!(
            parsed,
            Some(PathBuf::from("C:\\scoop\\apps\\git\\current\\bin\\git.exe"))
        );
    }

    #[test]
    fn test_parse_shim_file_empty_quoted_args() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        let mut file = File::create(&shim_path).unwrap();
        writeln!(file, "path = C:\\scoop\\apps\\git\\current\\bin\\git.exe").unwrap();
        writeln!(file, "args = \"\"").unwrap();
        let parsed = parse_shim_file(&shim_path);
        assert_eq!(
            parsed,
            Some(PathBuf::from("C:\\scoop\\apps\\git\\current\\bin\\git.exe"))
        );
    }

    #[test]
    fn test_parse_shim_file_comments_and_spaces() {
        let dir = tempdir().unwrap();
        let shim_path = dir.path().join("test.shim");
        let mut file = File::create(&shim_path).unwrap();
        writeln!(file, "# This is a comment").unwrap();
        writeln!(file, "  ; Another comment").unwrap();
        writeln!(file, "path   =   C:\\test.exe  ").unwrap();
        let parsed = parse_shim_file(&shim_path);
        assert_eq!(parsed, Some(PathBuf::from("C:\\test.exe")));
    }

    #[test]
    fn test_resolve_scoop_shim() {
        let dir = tempdir().unwrap();

        // Let's mock a scoop-like directory structure:
        // dir/
        //   shims/
        //     git.exe (empty placeholder)
        //     git.shim (path = ../apps/git/git.exe)
        //   apps/
        //     git/
        //       git.exe (empty placeholder)

        let shims_dir = dir.path().join("shims");
        fs::create_dir_all(&shims_dir).unwrap();

        let apps_dir = dir.path().join("apps").join("git");
        fs::create_dir_all(&apps_dir).unwrap();

        let git_exe_shim = shims_dir.join("git.exe");
        File::create(&git_exe_shim).unwrap();

        let git_shim_file = shims_dir.join("git.shim");
        {
            let mut file = File::create(&git_shim_file).unwrap();
            writeln!(file, "path = ../apps/git/git.exe").unwrap();
        }

        let actual_git_exe = apps_dir.join("git.exe");
        File::create(&actual_git_exe).unwrap();

        // Temporarily override SCOOP env var to point to our temp dir
        let original_scoop = env::var("SCOOP");
        unsafe {
            env::set_var("SCOOP", dir.path());
        }

        // Resolve
        let resolved = resolve_scoop_shim(git_exe_shim.clone());

        // Restore env var
        unsafe {
            match original_scoop {
                Ok(val) => env::set_var("SCOOP", val),
                Err(_) => env::remove_var("SCOOP"),
            }
        }

        // The resolved path should be canonicalized or verified
        // Since we resolved it relative to shims dir parent (which is dir/shims/../apps/git/git.exe = dir/apps/git/git.exe)
        // Let's compare canonical versions:
        let canonical_resolved = dunce::canonicalize(resolved).unwrap();
        let canonical_actual = dunce::canonicalize(actual_git_exe).unwrap();
        assert_eq!(canonical_resolved, canonical_actual);
    }
}
