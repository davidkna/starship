use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct OspConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for OspConfig<'a> {
    fn default() -> Self {
        OspConfig {
            format: "on [$symbol$cloud(\\($project\\))]($style) ",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
        }
    }
}
