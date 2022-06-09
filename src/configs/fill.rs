use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct FillConfig<'a> {
    /// The style for the module.
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for FillConfig<'a> {
    fn default() -> Self {
        FillConfig {
            style: "bold black",
            symbol: ".",
            disabled: false,
        }
    }
}
