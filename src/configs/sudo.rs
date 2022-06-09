use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct SudoConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub allow_windows: bool,
    pub disabled: bool,
}

impl<'a> Default for SudoConfig<'a> {
    fn default() -> Self {
        SudoConfig {
            format: "[as $symbol]($style)",
            symbol: "🧙 ",
            style: "bold blue",
            allow_windows: false,
            disabled: true,
        }
    }
}
