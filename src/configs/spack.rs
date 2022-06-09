use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct SpackConfig<'a> {
    pub truncation_length: usize,
    /// The format for the module.
    pub format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SpackConfig<'a> {
    fn default() -> Self {
        SpackConfig {
            truncation_length: 1,
            format: "via [$symbol$environment]($style) ",
            symbol: "🅢 ",
            style: "blue bold",
            disabled: false,
        }
    }
}
