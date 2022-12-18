use crate::currency_api;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Enum of asset categories.
#[derive(Deserialize, Serialize)]
pub enum AssetCategory {
    /// Unknown.
    Unknown,
    /// Stocks/bonds/etc.
    Stock,
    /// FIAT Currencies.
    Currency,
    /// Cryptocurrencies.
    Cryptocurrency,
}

/// General asset struct.
#[derive(Deserialize, Serialize)]
pub struct Asset {
    /// Category of asset.
    pub category: AssetCategory,
    /// Display name.
    pub name: String,
    /// Ticker.
    pub code: String,
    /// Amount.
    pub amount: f64,
    /// Value.
    pub value: Option<f64>,
}

impl Asset {
    /// Get value of asset in the given currency.
    pub fn get_value(&self, currency: &str) -> Result<f64> {
        match self.category {
            AssetCategory::Stock => {
                // ...
                Err(anyhow!("not implemented yet"))
            }
            AssetCategory::Currency => {
                let price = currency_api::get_value(&self.code.to_lowercase(), currency)?;
                Ok(price * self.amount)
            }
            AssetCategory::Cryptocurrency => {
                let price = currency_api::get_value(&self.code.to_lowercase(), currency)?;
                Ok(price * self.amount)
            }
            AssetCategory::Unknown => Err(anyhow!("category is unknown")),
        }
    }
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            category: AssetCategory::Unknown,
            name: String::new(),
            code: String::new(),
            amount: 0.0,
            value: None,
        }
    }
}
