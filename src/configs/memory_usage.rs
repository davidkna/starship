use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct MemoryConfig<'a> {
    pub threshold: i64,
    /// The format for the module.
    pub format: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for MemoryConfig<'a> {
    fn default() -> Self {
        MemoryConfig {
            threshold: 75,
            format: "via $symbol[$ram( | $swap)]($style) ",
            style: "white bold dimmed",
            symbol: "🐏 ",
            disabled: true,
        }
    }
}
