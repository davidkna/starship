wit_bindgen::generate!({
    world: "starship-module",
    path: "wit",
});

struct Component;

use exports::starship::wasm_interface::formatter::Guest;

impl Guest for Component {
    fn is_enabled() -> Result<bool, String> {
        Ok(true)
    }

    fn map(variable: String) -> Result<Option<String>, String> {
        match variable.as_str() {
            "output" => {
                // Get env var or compute something
                let user = std::env::var("USER").unwrap_or_else(|_| "unknown".into());
                Ok(Some(format!("WASM[{}]", user)))
            }
            _ => Ok(None),
        }
    }

    fn map_meta(variable: String) -> Result<Option<String>, String> {
        match variable.as_str() {
            "symbol" => Ok(Some("🦀".into())),
            _ => Ok(None),
        }
    }

    fn map_style(_variable: String) -> Result<Option<String>, String> {
        Ok(None) // Use default styles from config
    }
}

// Export the component
export!(Component);
