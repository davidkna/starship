use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct RubyConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
    pub detect_variables: Vec<&'a str>,
}

impl<'a> Default for RubyConfig<'a> {
    fn default() -> Self {
        RubyConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "💎 ",
            style: "bold red",
            disabled: false,
            detect_extensions: vec!["rb"],
            detect_files: vec!["Gemfile", ".ruby-version"],
            detect_folders: vec![],
            detect_variables: vec!["RUBY_VERSION", "RBENV_VERSION"],
        }
    }
}
