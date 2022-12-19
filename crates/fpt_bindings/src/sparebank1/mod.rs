use anyhow::Result;

use crate::common::*;
use fpt_common::*;

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

/// Get Sparebank 1 accounts and transform them into vector of [`Asset`].
pub fn get_assets(config: &mut Config) -> Result<Vec<Asset>> {
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

        out.push(Asset {
            category: AssetCategory::Currency,
            name: name.to_string(),
            code: currency.to_string(),
            amount: balance,
            value_usd: None,
            value_in_currency: None,
        });
    }

    Ok(out)
}
