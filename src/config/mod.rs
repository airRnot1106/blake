mod keymap;
mod loader;

pub use keymap::{KeyBinding, KeymapConfig};
pub use loader::ConfigLoader;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub keymap: KeymapConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            keymap: KeymapConfig::with_defaults(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralConfig {
    #[serde(default = "default_formatter")]
    pub diff_formatter: String,
}

fn default_formatter() -> String {
    "delta".to_string()
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            diff_formatter: default_formatter(),
        }
    }
}
