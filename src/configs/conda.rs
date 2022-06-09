use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct CondaConfig<'a> {
    pub truncation_length: usize,
    /// The format for the module.
    pub format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub ignore_base: bool,
    pub disabled: bool,
}

impl<'a> Default for CondaConfig<'a> {
    fn default() -> Self {
        CondaConfig {
            truncation_length: 1,
            format: "via [$symbol$environment]($style) ",
            symbol: "🅒 ",
            style: "green bold",
            ignore_base: true,
            disabled: false,
        }
    }
}
