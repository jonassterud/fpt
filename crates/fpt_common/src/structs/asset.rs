use serde::{Deserialize, Serialize};

/// Enum of asset categories.
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Asset {
    /// Category of asset.
    pub category: AssetCategory,
    /// Display name.
    pub name: String,
    /// Ticker.
    pub code: String,
    /// Amount.
    pub amount: f64,
    /// Value in USD.
    pub value_usd: Option<f64>,
    /// Value in the given currency.
    pub value_in_currency: Option<f64>,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            category: AssetCategory::Unknown,
            name: String::new(),
            code: String::new(),
            amount: 0.0,
            value_usd: None,
            value_in_currency: None,
        }
    }
}
