use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct PulumiConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
    pub search_upwards: bool,
}

impl<'a> Default for PulumiConfig<'a> {
    fn default() -> Self {
        PulumiConfig {
            format: "via [$symbol($username@)$stack]($style) ",
            version_format: "v${raw}",
            symbol: " ",
            style: "bold 5",
            disabled: false,
            search_upwards: true,
        }
    }
}
