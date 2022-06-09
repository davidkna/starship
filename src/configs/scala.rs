use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct ScalaConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
    pub disabled: bool,
    /// The style for the module.
    pub style: &'a str,
    pub symbol: &'a str,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ScalaConfig<'a> {
    fn default() -> Self {
        ScalaConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            disabled: false,
            style: "red bold",
            symbol: "🆂 ",
            detect_extensions: vec!["sbt", "scala"],
            detect_files: vec![".scalaenv", ".sbtenv", "build.sbt"],
            detect_folders: vec![".metals"],
        }
    }
}
