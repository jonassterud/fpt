use crate::Database;
use actix_web::{get, Responder};
use anyhow::Result;

/// Get assets from the database.
#[get("/get_assets")]
pub async fn get_assets() -> impl Responder {
    fn open_and_get_assets() -> Result<String> {
        let db = Database::open()?;
        let assets = db.get_assets()?;

        Ok(serde_json::to_string(&assets)?)
    }

    match open_and_get_assets() {
        Ok(out) => actix_web::HttpResponse::Ok().body(out),
        Err(e) => {
            eprint!("{}", e);
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
