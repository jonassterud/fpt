use crate::{sparebank1_api, Database};
use actix_web::{get, Responder};
use anyhow::Result;

/// Updates assets and adds them to the database.
#[get("/update_assets")]
pub async fn update_assets() -> impl Responder {
    fn add_assets_to_database() -> Result<()> {
        let db = Database::open()?;

        let mut assets = vec![];
        assets.append(&mut sparebank1_api::get_assets()?);

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
