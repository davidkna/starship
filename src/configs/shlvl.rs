use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct ShLvlConfig<'a> {
    pub threshold: i64,
    /// The format for the module.
    pub format: &'a str,
    pub symbol: &'a str,
    pub repeat: bool,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for ShLvlConfig<'a> {
    fn default() -> Self {
        ShLvlConfig {
            threshold: 2,
            format: "[$symbol$shlvl]($style) ",
            symbol: "↕️  ", // extra space for emoji
            repeat: false,
            style: "bold yellow",
            disabled: true,
        }
    }
}
