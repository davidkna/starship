use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct BatteryConfig<'a> {
    /// The symbol shown when the battery is full.  
    pub full_symbol: &'a str,
    /// The symbol shown when the battery is charging.
    pub charging_symbol: &'a str,
    /// The symbol shown when the battery is discharging.
    pub discharging_symbol: &'a str,
    /// The symbol shown when the battery state is unknown.
    pub unknown_symbol: &'a str,
    /// The symbol shown when the battery state is empty.
    pub empty_symbol: &'a str,
    /// Display threshold and style for the module.
    #[serde(borrow)]
    pub display: Vec<BatteryDisplayConfig<'a>>,
    /// Disables the `battery` module.
    pub disabled: bool,
    /// The format for the module.
    pub format: &'a str,
}

impl<'a> Default for BatteryConfig<'a> {
    fn default() -> Self {
        BatteryConfig {
            full_symbol: " ",
            charging_symbol: " ",
            discharging_symbol: " ",
            unknown_symbol: " ",
            empty_symbol: " ",
            format: "[$symbol$percentage]($style) ",
            display: vec![BatteryDisplayConfig::default()],
            disabled: false,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct BatteryDisplayConfig<'a> {
    /// The upper bound for the display
    pub threshold: i64,
    /// The style used if the display option is in use.
    /// The style for the module.
    pub style: &'a str,
    /// Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.
    pub charging_symbol: Option<&'a str>,
    /// Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option.
    pub discharging_symbol: Option<&'a str>,
}

impl<'a> Default for BatteryDisplayConfig<'a> {
    fn default() -> Self {
        BatteryDisplayConfig {
            threshold: 10,
            style: "red bold",
            charging_symbol: None,
            discharging_symbol: None,
        }
    }
}
