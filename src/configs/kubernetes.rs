use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct KubernetesConfig<'a> {
    pub symbol: &'a str,
    /// The format for the module.
    pub format: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
    pub context_aliases: HashMap<String, &'a str>,
    pub user_aliases: HashMap<String, &'a str>,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for KubernetesConfig<'a> {
    fn default() -> Self {
        KubernetesConfig {
            symbol: "☸ ",
            format: "[$symbol$context( \\($namespace\\))]($style) in ",
            style: "cyan bold",
            disabled: true,
            context_aliases: HashMap::new(),
            user_aliases: HashMap::new(),
            detect_extensions: vec![],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
