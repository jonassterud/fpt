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

/// Get balance of Ethereum address.
pub fn get_balance(address: &str) -> Result<f64> {
    let url = format!("https://api.ethplorer.io/getAddressInfo/{address}?apiKey=freekey");
    let resp = ureq::get(&url)
        .set("Accept", "application/json; charset=utf-8")
        .call()?
        .into_json::<serde_json::Value>()?;

    let out = gv_f64("balance", gv("ETH", &resp)?)?;

    Ok(out)
}
