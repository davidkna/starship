use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct BufConfig<'a> {
    /// The format for the `buf` module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
    /// The symbol used before displaying the version of Buf.
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    /// Disables the `elixir` module.
    pub disabled: bool,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for BufConfig<'a> {
    fn default() -> Self {
        BufConfig {
            format: "with [$symbol ($version)]($style)",
            version_format: "v${raw}",
            symbol: "",
            style: "bold blue",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["buf.yaml", "buf.gen.yaml", "buf.work.yaml"],
            detect_folders: vec![],
        }
    }
}
