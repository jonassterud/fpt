use crate::{config::Config, sparebank1_api, Database};
use actix_web::{get, Responder};
use anyhow::Result;

/// Updates assets and adds them to the database.
#[get("/update_assets")]
pub async fn update_assets() -> impl Responder {
    fn add_assets_to_database() -> Result<()> {
        let db = Database::open()?;
        let mut config = Config::load()?;

        let mut assets = vec![];

        if !config.sparebank1_id.is_empty()
            && !config.sparebank1_secret.is_empty()
            && !config.sparebank1_refresh_token.is_empty()
        {
            assets.append(&mut sparebank1_api::get_assets(&mut config)?);
        }

        // TODO: Don't loop - insert all assets in one query instead.
        for asset in assets {
            db.add_asset(&asset)?;
        }

        Ok(())
    }

    match add_assets_to_database() {
        Ok(()) => actix_web::HttpResponse::Ok().finish(),
        Err(e) => {
            eprint!("{}", e);
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
