use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct DockerContextConfig<'a> {
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// The format for the module.
    pub format: &'a str,
    pub only_with_files: bool,
    pub disabled: bool,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for DockerContextConfig<'a> {
    fn default() -> Self {
        DockerContextConfig {
            symbol: "🐳 ",
            style: "blue bold",
            format: "via [$symbol$context]($style) ",
            only_with_files: true,
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["docker-compose.yml", "docker-compose.yaml", "Dockerfile"],
            detect_folders: vec![],
        }
    }
}
