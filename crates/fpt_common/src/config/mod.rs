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
    /// Sparebank 1 refresh token.
    pub sparebank1_refresh_token: String,
    /// Bitcoin addresses.
    pub bitcoin_addresses: Vec<String>,
    /// Ethereum addresses.
    pub ethereum_addresses: Vec<String>,
}

impl Config {
    /// Open/init `config.toml`.
    pub fn load() -> Result<Config> {
        let config = if let Ok(bytes) = std::fs::read("config.toml") {
            toml::from_slice(&bytes)?
        } else {
            Config::default()
        };

        config.write()?;

        Ok(config)
    }

    /// Write [`Config`] to `config.toml`.
    pub fn write(&self) -> Result<()> {
        std::fs::write("config.toml", toml::to_string_pretty(self)?)?;

        Ok(())
    }
}
