use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitTagConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub only_detached: bool,
    pub max_candidates: usize,
}

impl Default for GitTagConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$symbol$tag]($style) ",
            style: "green bold",
            symbol: "🏷  ",
            disabled: true,
            only_detached: false,
            max_candidates: 0,
        }
    }
}
