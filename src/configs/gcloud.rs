use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct GcloudConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
    pub region_aliases: HashMap<String, &'a str>,
    pub project_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for GcloudConfig<'a> {
    fn default() -> Self {
        GcloudConfig {
            format: "on [$symbol$account(@$domain)(\\($region\\))]($style) ",
            symbol: "☁️  ",
            style: "bold blue",
            disabled: false,
            region_aliases: HashMap::new(),
            project_aliases: HashMap::new(),
        }
    }
}
