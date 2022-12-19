use crate::Database;
use actix_web::{get, Responder};
use anyhow::Result;

/// Updates assets and adds them to the database.
#[get("/update_values")]
pub async fn update_values() -> impl Responder {
    let update_values_in_database = move || -> Result<()> {
        let db = Database::open()?;
        let mut assets = db.get_assets()?;

        db.clear_assets()?;
        for asset in &mut assets {
            asset.value_usd = Some(asset.get_value("usd")?);
            db.add_asset(asset)?;
        }

        Ok(())
    };

    match update_values_in_database() {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(error) => {
            eprint!("{error}");
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
