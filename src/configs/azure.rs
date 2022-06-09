use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct AzureConfig<'a> {
    /// The format for the Azure module to render.
    pub format: &'a str,
    /// The symbol used in the format.  
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// Disables the `azure` module.
    pub disabled: bool,
}

impl<'a> Default for AzureConfig<'a> {
    fn default() -> Self {
        AzureConfig {
            format: "on [$symbol($subscription)]($style) ",
            symbol: "ﴃ ",
            style: "blue bold",
            disabled: true,
        }
    }
}
