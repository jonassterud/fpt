use anyhow::Result;

use crate::common::*;

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
