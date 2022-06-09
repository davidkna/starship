use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct SingularityConfig<'a> {
    pub symbol: &'a str,
    /// The format for the module.
    pub format: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SingularityConfig<'a> {
    fn default() -> Self {
        SingularityConfig {
            format: "[$symbol\\[$env\\]]($style) ",
            symbol: "",
            style: "blue bold dimmed",
            disabled: false,
        }
    }
}
