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
}

impl Asset {
    /// Get value of asset in the given currency.
    pub fn get_value(&self, currency: &str) -> Result<i64> {
        match self.category {
            AssetCategory::Stock => {
                let stock_price = todo!();
                stock_price
            }
            AssetCategory::Currency => {
                let currency_price = todo!();
                currency_price
            }
            AssetCategory::Cryptocurrency => {
                let cryptocurrency_price = todo!();
                cryptocurrency_price
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
        }
    }
}
