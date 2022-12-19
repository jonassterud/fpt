use anyhow::Result;

use crate::common::*;

/// Get balance of Bitcoin address.
pub fn get_balance(address: &str) -> Result<f64> {
    let url = format!("https://chain.api.btc.com/v3/address/{address}");
    let resp = ureq::get(&url)
        .set("Accept", "application/json; charset=utf-8")
        .call()?
        .into_json::<serde_json::Value>()?;

    let out = gv_f64("balance", gv("data", &resp)?)? / 100000000.0;

    Ok(out)
}
