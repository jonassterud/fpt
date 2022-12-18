use crate::{currency_api, Database};
use actix_web::{get, web, Responder};
use anyhow::Result;

/// Get assets from the database.
#[get("/get_assets/{currency}")]
pub async fn get_assets(currency: web::Path<String>) -> impl Responder {
    let open_and_get_assets = move || -> Result<String> {
        let db = Database::open()?;
        let mut assets = db.get_assets()?;

        for asset in &mut assets {
            let value_usd = asset.value_usd.unwrap_or_default();
            asset.value_in_currency = Some(value_usd * currency_api::get_value("usd", &currency)?);
        }

        Ok(serde_json::to_string(&assets)?)
    };

    match open_and_get_assets() {
        Ok(out) => actix_web::HttpResponse::Ok().body(out),
        Err(error) => {
            eprint!("{error}");
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
