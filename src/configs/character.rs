use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct CharacterConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The format string used before the text input if the previous command succeeded.
    pub success_symbol: &'a str,
    /// The format string used before the text input if the previous command failed.
    pub error_symbol: &'a str,
    /// The format string used before the text input if the shell is in vim normal mode.
    #[serde(alias = "vicmd_symbol")]
    pub vimcmd_symbol: &'a str,
    //  The format string used before the text input if the shell is in vim replace mode.
    pub vimcmd_visual_symbol: &'a str,
    /// he format string used before the text input if the shell is in vim replace mode.
    pub vimcmd_replace_symbol: &'a str,
    /// The format string used before the text input if the shell is in vim `replace_one` mode.
    pub vimcmd_replace_one_symbol: &'a str,
    /// Disables the `character` module.
    pub disabled: bool,
}

impl<'a> Default for CharacterConfig<'a> {
    fn default() -> Self {
        CharacterConfig {
            format: "$symbol ",
            success_symbol: "[❯](bold green)",
            error_symbol: "[❯](bold red)",
            vimcmd_symbol: "[❮](bold green)",
            vimcmd_visual_symbol: "[❮](bold yellow)",
            vimcmd_replace_symbol: "[❮](bold purple)",
            vimcmd_replace_one_symbol: "[❮](bold purple)",
            disabled: false,
        }
    }
}
