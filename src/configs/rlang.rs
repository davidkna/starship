use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct RLangConfig<'a> {
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

impl<'a> Default for RLangConfig<'a> {
    fn default() -> Self {
        RLangConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            style: "blue bold",
            symbol: "📐 ",
            disabled: false,
            detect_extensions: vec!["R", "Rd", "Rmd", "Rproj", "Rsx"],
            detect_files: vec![".Rprofile"],
            detect_folders: vec![".Rproj.user"],
        }
    }
}
