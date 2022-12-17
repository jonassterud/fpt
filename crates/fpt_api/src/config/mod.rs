use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Config struct.
#[derive(Default, Deserialize, Serialize)]
#[serde(default, rename_all = "UPPERCASE")]
pub struct Config {
    /// Sparebank 1 client ID.
    pub sparebank1_id: String,
    /// Sparebank 1 client secret.
    pub sparebank1_secret: String,
    /// Sparbank 1 refresh token.
    pub sparebank1_refresh_token: String,
}

impl Config {
    /// Open/init `config.toml`.
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
