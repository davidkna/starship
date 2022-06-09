use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct VcshConfig<'a> {
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// The format for the module.
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> Default for VcshConfig<'a> {
    fn default() -> Self {
        VcshConfig {
            symbol: "",
            style: "bold yellow",
            format: "vcsh [$symbol$repo]($style) ",
            disabled: false,
        }
    }
}
