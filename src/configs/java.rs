use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct JavaConfig<'a> {
    pub disabled: bool,
    /// The format for the module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
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

impl<'a> Default for JavaConfig<'a> {
    fn default() -> Self {
        JavaConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            disabled: false,
            style: "red dimmed",
            symbol: "☕ ",
            detect_extensions: vec!["java", "class", "jar", "gradle", "clj", "cljc"],
            detect_files: vec![
                "pom.xml",
                "build.gradle.kts",
                "build.sbt",
                ".java-version",
                "deps.edn",
                "project.clj",
                "build.boot",
            ],
            detect_folders: vec![],
        }
    }
}
