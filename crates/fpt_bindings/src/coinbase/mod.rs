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
pub fn get_assets(config: &Config) -> Result<f64> {
    let key = get_access_key(config);
    let timestamp = get_access_timestamp()?;
    let sign = get_access_sign(config, timestamp, "GET", "/api/v3/brokerage/accounts/", "")?;

    let url = "https://api.coinbase.com/api/v3/brokerage/accounts/";
    let resp = ureq::get(url)
        .set("Accept", "application/json; charset=utf-8")
        .call()?
        .into_json::<serde_json::Value>()?;

    todo!();
}
