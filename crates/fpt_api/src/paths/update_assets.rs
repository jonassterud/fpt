use crate::Database;
use actix_web::{get, Responder};
use anyhow::Result;
use fpt_bindings::*;
use fpt_common::*;

/// Updates assets and adds them to the database.
#[get("/update_assets")]
pub async fn update_assets() -> impl Responder {
    let add_assets_to_database = || -> Result<()> {
        let mut out = Ok(());

        let db = Database::open()?;
        let mut config = Config::load()?;

        let mut assets = vec![];

        // Load Sparebank 1 assets
        if !config.sparebank1_id.is_empty()
            && !config.sparebank1_secret.is_empty()
            && !config.sparebank1_refresh_token.is_empty()
        {
            match sparebank1::get_assets(&mut config) {
                Ok(mut out) => assets.append(&mut out),
                Err(error) => out = Err(error),
            };
        }

        // Load Coinbase assets
        if !config.coinbase_key.is_empty() && !config.coinbase_secret.is_empty() {
            match coinbase::get_assets(&config) {
                Ok(mut out) => assets.append(&mut out),
                Err(error) => out = Err(error),
            };
        }

        // Load Bitcoin assets
        for bitcoin_address in config.bitcoin_addresses {
            assets.push(Asset {
                category: AssetCategory::Cryptocurrency,
                name: "Bitcoin".to_string(),
                code: "BTC".to_string(),
                amount: btccom::get_balance(&bitcoin_address)?,
                value_usd: None,
                value_in_currency: None,
            });
        }

        // Load Ethereum assets
        for ethereum_address in config.ethereum_addresses {
            assets.push(Asset {
                category: AssetCategory::Cryptocurrency,
                name: "Ethereum".to_string(),
                code: "ETH".to_string(),
                amount: ethplorer::get_balance(&ethereum_address)?,
                value_usd: None,
                value_in_currency: None,
            });
        }

        // TODO: Don't loop - insert all assets in one query instead.
        db.clear_assets()?;
        for asset in assets {
            db.add_asset(&asset)?;
        }

        out
    };

    match add_assets_to_database() {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(error) => {
            log::error!("{error}");
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
