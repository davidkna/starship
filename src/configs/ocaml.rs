use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct OCamlConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
    pub global_switch_indicator: &'a str,
    pub local_switch_indicator: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub disabled: bool,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for OCamlConfig<'a> {
    fn default() -> Self {
        OCamlConfig {
            format: "via [$symbol($version )(\\($switch_indicator$switch_name\\) )]($style)",
            version_format: "v${raw}",
            global_switch_indicator: "",
            local_switch_indicator: "*",
            symbol: "🐫 ",
            style: "bold yellow",
            disabled: false,
            detect_extensions: vec!["opam", "ml", "mli", "re", "rei"],
            detect_files: vec!["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"],
            detect_folders: vec!["_opam", "esy.lock"],
        }
    }
}
