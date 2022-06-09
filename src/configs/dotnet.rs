use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct DotnetConfig<'a> {
    /// The format for the module.
    pub format: &'a str,
    /// The version format. Available vars are `raw`, `major`, `minor`, & `patch`
    pub version_format: &'a str,
    pub symbol: &'a str,
    /// The style for the module.
    pub style: &'a str,
    pub heuristic: bool,
    pub disabled: bool,
    /// Which extensions should trigger this module.
    pub detect_extensions: Vec<&'a str>,
    /// Which filenames should trigger this module.
    pub detect_files: Vec<&'a str>,
    /// Which folders should trigger this module.
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for DotnetConfig<'a> {
    fn default() -> Self {
        DotnetConfig {
            format: "via [$symbol($version )(🎯 $tfm )]($style)",
            version_format: "v${raw}",
            symbol: ".NET ",
            style: "blue bold",
            heuristic: true,
            disabled: false,
            detect_extensions: vec!["csproj", "fsproj", "xproj"],
            detect_files: vec![
                "global.json",
                "project.json",
                "Directory.Build.props",
                "Directory.Build.targets",
                "Packages.props",
            ],
            detect_folders: vec![],
        }
    }
}
