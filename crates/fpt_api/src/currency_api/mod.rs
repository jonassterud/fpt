use crate::{config::Config, structures};
use anyhow::{anyhow, Result};

/// Get value from `serde_json::Value` as a `&serde_json::Value`.
fn gv<'a>(key: &str, val: &'a serde_json::Value) -> Result<&'a serde_json::Value> {
    val.get(key)
        .ok_or_else(|| anyhow!("value is missing {key}"))
}

/// Get value from `serde_json::Value` as a `&f64`.
fn gv_f64(key: &str, val: &serde_json::Value) -> Result<f64> {
    gv(key, val)?
        .as_f64()
        .ok_or_else(|| anyhow!("value is not a f64"))
}

/// Get value between two currencies.
pub fn get_value(from: &str, to: &str) -> Result<f64> {
    let url = format!(
        "https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/{from}/{to}.json"
    );
    let resp = ureq::get(&url)
        .set("Accept", "application/json; charset=utf-8")
        .call()?
        .into_json::<serde_json::Value>()?;

    let out = gv_f64(to, &resp)?;

    Ok(out)
}
