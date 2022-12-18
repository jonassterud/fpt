use crate::{btccom_api, config::Config, sparebank1_api, structures, Database};
use actix_web::{get, Responder};
use anyhow::Result;

/// Updates assets and adds them to the database.
#[get("/update_assets")]
pub async fn update_assets() -> impl Responder {
    fn add_assets_to_database() -> Result<()> {
        let db = Database::open()?;
        let mut config = Config::load()?;

        let mut assets = vec![];

        // Load Sparebank 1 assets
        if !config.sparebank1_id.is_empty()
            && !config.sparebank1_secret.is_empty()
            && !config.sparebank1_refresh_token.is_empty()
        {
            assets.append(&mut sparebank1_api::get_assets(&mut config)?);
        }

        // Load Bitcoin assets
        for bitcoin_address in config.bitcoin_addresses {
            assets.push(structures::Asset {
                category: structures::AssetCategory::Cryptocurrency,
                name: "Bitcoin".to_string(),
                code: "BTC".to_string(),
                amount: btccom_api::get_balance(&bitcoin_address)?,
                value: None,
            });
        }

        // TODO: Don't loop - insert all assets in one query instead.
        db.clear_assets()?;
        for asset in assets {
            db.add_asset(&asset)?;
        }

        Ok(())
    }

    match add_assets_to_database() {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(error) => {
            eprint!("{error}");
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
