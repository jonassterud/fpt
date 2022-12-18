use crate::{config::Config, structures};
use anyhow::{anyhow, Result};

/// Get value from `serde_json::Value` as a `&serde_json::Value`.
fn gv<'a>(key: &str, val: &'a serde_json::Value) -> Result<&'a serde_json::Value> {
    val.get(key)
        .ok_or_else(|| anyhow!("value is missing {key}"))
}

/// Get value from `serde_json::Value` as a `&str`.
fn gv_str<'a>(key: &str, val: &'a serde_json::Value) -> Result<&'a str> {
    gv(key, val)?
        .as_str()
        .ok_or_else(|| anyhow!("value is not a str"))
}

/// Get value from `serde_json::Value` as a `&Vec<serde_json::Value>`.
fn gv_vec<'a>(key: &str, val: &'a serde_json::Value) -> Result<&'a Vec<serde_json::Value>> {
    gv(key, val)?
        .as_array()
        .ok_or_else(|| anyhow!("value is not an array"))
}

/// Get value from `serde_json::Value` as a `&f64`.
fn gv_f64(key: &str, val: &serde_json::Value) -> Result<f64> {
    gv(key, val)?
        .as_f64()
        .ok_or_else(|| anyhow!("value is not a f64"))
}

/// Get access token from refresh token.
pub fn get_access_token(config: &mut Config) -> Result<String> {
    let url = "https://api-auth.sparebank1.no/oauth/token";
    let resp = ureq::post(url)
        .send_form(&[
            ("client_id", &config.sparebank1_id),
            ("client_secret", &config.sparebank1_secret),
            ("refresh_token", &config.sparebank1_refresh_token),
            ("grant_type", "refresh_token"),
        ])?
        .into_json::<serde_json::Value>()?;

    let new_access_token = gv_str("access_token", &resp)?;
    let new_refresh_token = gv_str("refresh_token", &resp)?;

    config.sparebank1_refresh_token = new_refresh_token.to_string();
    config.write()?;

    Ok(new_access_token.to_string())
}

/// Get Sparebank 1 accounts and transform them into vector of [`structures::Asset`].
pub fn get_assets(config: &mut Config) -> Result<Vec<structures::Asset>> {
    let mut out = vec![];

    let resp = ureq::get("https://api.sparebank1.no/personal/banking/accounts")
        .set(
            "Accept",
            "application/vnd.sparebank1.v5+json; charset=utf-8",
        )
        .set(
            "Authorization",
            &format!("Bearer {}", get_access_token(config)?),
        )
        .call()?
        .into_json::<serde_json::Value>()?;

    for account in gv_vec("accounts", &resp)? {
        let name = gv_str("name", account)?;
        let balance = gv_f64("balance", account)?;
        let currency = gv_str("currencyCode", account)?;

        out.push(structures::Asset {
            category: structures::AssetCategory::Currency,
            name: name.to_string(),
            code: currency.to_string(),
            amount: balance,
            value_usd: None,
            value_in_currency: None,
        });
    }

    Ok(out)
}
