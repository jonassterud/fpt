use crate::{config::Config, structures};
use anyhow::{anyhow, Result};

/// Get value from `serde_json::Value` as a `&f64`.
fn gv_f64(key: &str, val: &serde_json::Value) -> Result<f64> {
    val.get(key)
        .ok_or_else(|| anyhow!("value is missing {key}"))?
        .as_f64()
        .ok_or_else(|| anyhow!("value is not a f64"))
}

/// Get price of crypto currency.
pub fn get_price(id: &str) -> Result<f64> {
    let url = format!("https://api.coinpaprika.com/v1/coins/{id}/ohlcv/today");
    let resp = ureq::get(&url)
        .set("Accept", "application/json; charset=utf-8")
        .call()?
        .into_json::<serde_json::Value>()?;

    let out = gv_f64(
        "close",
        resp.get(0).ok_or_else(|| anyhow!("value is not a vec"))?,
    )?;

    Ok(out)
}
