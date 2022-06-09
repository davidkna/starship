use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct RakuConfig<'a> {
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
}

impl<'a> Default for RakuConfig<'a> {
    fn default() -> Self {
        RakuConfig {
            format: "via [$symbol($version-$vm_version )]($style)",
            version_format: "${raw}",
            symbol: "🦋 ",
            style: "149 bold",
            disabled: false,
            detect_extensions: vec!["p6", "pm6", "pod6", "raku", "rakumod"],
            detect_files: vec!["META6.json"],
            detect_folders: vec![],
        }
    }
}
