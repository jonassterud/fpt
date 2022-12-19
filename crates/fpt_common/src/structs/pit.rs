use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// "Point in time" struct.
#[derive(Deserialize, Serialize)]
pub struct Pit {
    /// UNIX time
    pub time: u64,
    /// Total value of assets.
    pub total_value_usd: f64,
    /// Total value in currency.
    pub total_value_in_currency: Option<f64>,
}

impl Pit {
    /// Create point in time.
    pub fn new(total_value_usd: f64) -> Result<Self> {
        Ok(Self {
            time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            total_value_usd,
            total_value_in_currency: None,
        })
    }
}
