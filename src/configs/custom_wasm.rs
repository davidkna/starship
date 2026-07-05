use crate::config::{Either, VecOr};

use serde::{self, Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct CustomWasmConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub wasm_path: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    #[serde(alias = "files")]
    pub detect_files: Vec<&'a str>,
    #[serde(alias = "extensions")]
    pub detect_extensions: Vec<&'a str>,
    #[serde(alias = "directories")]
    pub detect_folders: Vec<&'a str>,
    pub when: Either<bool, &'a str>,
    pub require_repo: bool,
    pub shell: VecOr<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stdin: Option<bool>,
    pub ignore_timeout: bool,
    pub description: &'a str,
}

impl Default for CustomWasmConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$symbol$output]($style) ",
            symbol: "🧩 ",
            wasm_path: "",
            style: "bold blue",
            disabled: false,
            detect_files: Vec::default(),
            detect_extensions: Vec::default(),
            detect_folders: Vec::default(),
            when: Either::First(false),
            require_repo: false,
            shell: VecOr::default(),
            os: None,
            use_stdin: None,
            ignore_timeout: false,
            description: "Custom WASM module",
        }
    }
}
