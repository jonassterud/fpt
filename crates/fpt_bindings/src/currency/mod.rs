use anyhow::Result;

use crate::common::*;

/// Get value of asset.
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
