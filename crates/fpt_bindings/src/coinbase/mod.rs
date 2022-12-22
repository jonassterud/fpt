use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::*;
use fpt_common::*;

fn get_access_key(config: &Config) -> &str {
    &config.coinbase_key
}

fn get_access_sign(
    config: &Config,
    timestamp: u64,
    method: &str,
    request_path: &str,
    body: &str,
) -> Result<String> {
    let message = format!("{}{}{}{}", timestamp, method, request_path, body);
    let secret_decoded = base64::decode(&config.coinbase_secret)?;
    let mut hmac: Hmac<Sha256> = Hmac::new_from_slice(&secret_decoded)?;

    hmac.update(message.as_bytes());
    let out = base64::encode(hmac.finalize().into_bytes());

    Ok(out)
}

fn get_access_timestamp() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

/// Get Coinbase accounts and transform them into vector of [`Asset`].
pub fn get_assets(config: &Config) -> Result<Vec<Asset>> {
    let mut out = vec![];

    let key = get_access_key(config);
    let timestamp = get_access_timestamp()?;
    let sign = get_access_sign(config, timestamp, "GET", "/api/v3/brokerage/accounts", "")?;

    let url = "https://api.coinbase.com/api/v3/brokerage/accounts/";
    let resp = ureq::get(url)
        .set("Accept", "application/json; charset=utf-8")
        .set("CB-ACCESS-KEY", key)
        .set("CB-ACCESS-SIGN", &sign)
        .set("CB-ACCESS-TIMESTAMP", &timestamp.to_string())
        .set("User-Agent", "request")
        .call()?
        .into_json::<serde_json::Value>()?;

    for account in gv_vec("accounts", &resp)? {
        let name = gv_str("name", account)?;
        let available_balance = gv("available_balance", account)?;
        let value = gv_str("balance", available_balance)?.parse::<f64>()?;
        let currency = gv_str("currency", available_balance)?;

        out.push(Asset {
            category: AssetCategory::Cryptocurrency,
            name: name.to_string(),
            code: currency.to_string(),
            amount: value,
            value_usd: None,
            value_in_currency: None,
        });
    }

    Ok(out)
}
