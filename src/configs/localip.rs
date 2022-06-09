use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct LocalipConfig<'a> {
    pub ssh_only: bool,
    /// The format for the module.
    pub format: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for LocalipConfig<'a> {
    fn default() -> Self {
        LocalipConfig {
            ssh_only: true,
            format: "[$localipv4]($style) ",
            style: "yellow bold",
            disabled: true,
        }
    }
}
