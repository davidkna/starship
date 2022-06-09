use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct JobsConfig<'a> {
    pub threshold: i64,
    pub symbol_threshold: i64,
    pub number_threshold: i64,
    /// The format for the module.
    pub format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for JobsConfig<'a> {
    fn default() -> Self {
        JobsConfig {
            threshold: 1,
            symbol_threshold: 1,
            number_threshold: 2,
            format: "[$symbol$number]($style) ",
            symbol: "✦",
            style: "bold blue",
            disabled: false,
        }
    }
}
