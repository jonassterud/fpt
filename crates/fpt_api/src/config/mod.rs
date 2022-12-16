use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
#[serde(default, rename_all = "UPPERCASE")]
pub struct Config {
    pub sparebank1_id: String,
    pub sparebank1_secret: String,
    pub sparebank1_refresh_token: String,
}

impl Config {
    /// Create/load `config.toml`.
    pub fn load() -> Result<Config> {
        let config = if let Ok(bytes) = std::fs::read("config.toml") {
            toml::from_slice(&bytes)?
        } else {
            Config::default()
        };

        std::fs::write("config.toml", toml::to_string_pretty(&config)?)?;

        Ok(config)
    }

    /// Write [`Config`] to `config.toml`.
    pub fn write(&self) -> Result<()> {
        std::fs::write("config.toml", toml::to_string_pretty(self)?)?;

        Ok(())
    }
}
