use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct CConfig<'a> {
    /// The format string for the module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// The symbol used before displaying the compiler details
    pub symbol: &'a str,
    /// Disables the `c` module.
    pub disabled: bool,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
    /// How to detect what the compiler is
    pub commands: Vec<Vec<&'a str>>,
}

impl<'a> Default for CConfig<'a> {
    fn default() -> Self {
        CConfig {
            format: "via [$symbol($version(-$name) )]($style)",
            version_format: "v${raw}",
            style: "149 bold",
            symbol: "C ",
            disabled: false,
            detect_extensions: vec!["c", "h"],
            detect_files: vec![],
            detect_folders: vec![],
            commands: vec![
                // the compiler is usually cc, and --version works on gcc and clang
                vec!["cc", "--version"],
                // but on some platforms gcc is installed as *gcc*, not cc
                vec!["gcc", "--version"],
                // for completeness, although I've never seen a clang that wasn't cc
                vec!["clang", "--version"],
            ],
        }
    }
}
