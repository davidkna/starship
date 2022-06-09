use crate::config::VecOr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct PythonConfig<'a> {
    pub pyenv_version_name: bool,
    pub pyenv_prefix: &'a str,
    pub python_binary: VecOr<&'a str>,
    /// The format for the module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for PythonConfig<'a> {
    fn default() -> Self {
        PythonConfig {
            pyenv_version_name: false,
            pyenv_prefix: "pyenv ",
            python_binary: VecOr(vec!["python", "python3", "python2"]),
            format: "via [${symbol}${pyenv_prefix}(${version} )(\\($virtualenv\\) )]($style)",
            version_format: "v${raw}",
            style: "yellow bold",
            symbol: "🐍 ",
            disabled: false,
            detect_extensions: vec!["py"],
            detect_files: vec![
                "requirements.txt",
                ".python-version",
                "pyproject.toml",
                "Pipfile",
                "tox.ini",
                "setup.py",
                "__init__.py",
            ],
            detect_folders: vec![],
        }
    }
}
