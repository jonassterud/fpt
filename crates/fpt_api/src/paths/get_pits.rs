use actix_web::{get, web, Responder};
use anyhow::Result;

use crate::Database;
use fpt_bindings::*;

/// Get PITs from the database.
#[get("/get_pits/{currency}")]
pub async fn get_pits(currency: web::Path<String>) -> impl Responder {
    let open_and_get_pits = move || -> Result<String> {
        let db = Database::open()?;
        let mut pits = db.get_pits()?;

        for pit in &mut pits {
            let total_value_usd = pit.total_value_usd;
            pit.total_value_in_currency =
                Some(total_value_usd * currency::get_value("usd", &currency)?);
        }

        Ok(serde_json::to_string(&pits)?)
    };

    match open_and_get_pits() {
        Ok(out) => actix_web::HttpResponse::Ok().body(out),
        Err(error) => {
            log::error!("{error}");
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
