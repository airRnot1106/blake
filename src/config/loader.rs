use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use super::{AppConfig, KeymapConfig};

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> Result<AppConfig> {
        let path = Self::config_path();

        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let mut config: AppConfig = toml::from_str(&content)?;

            // Merge with defaults for missing keys
            let defaults = KeymapConfig::with_defaults();
            for (k, v) in defaults.global {
                config.keymap.global.entry(k).or_insert(v);
            }
            for (k, v) in defaults.blame {
                config.keymap.blame.entry(k).or_insert(v);
            }
            for (k, v) in defaults.diff {
                config.keymap.diff.entry(k).or_insert(v);
            }
            for (k, v) in defaults.help {
                config.keymap.help.entry(k).or_insert(v);
            }

            Ok(config)
        } else {
            Ok(AppConfig::default())
        }
    }

    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("blake")
            .join("config.toml")
    }
}
