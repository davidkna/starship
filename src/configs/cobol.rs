use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct CobolConfig<'a> {
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

impl<'a> Default for CobolConfig<'a> {
    fn default() -> Self {
        CobolConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "⚙️ ",
            style: "bold blue",
            disabled: false,
            detect_extensions: vec!["cbl", "cob", "CBL", "COB"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
