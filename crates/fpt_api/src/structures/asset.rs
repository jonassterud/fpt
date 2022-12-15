use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum AssetCategory {
    Unknown,
    Stock,
    Currency,
    Cryptocurrency,
}

#[derive(Deserialize, Serialize)]
pub struct Asset {
    pub category: AssetCategory,
    pub code: String,
    pub amount: f64,
}

impl Asset {
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
            code: String::new(),
            amount: 0.0,
        }
    }
}
