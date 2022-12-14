use actix_web::{get, Responder};
use anyhow::Result;

use crate::Database;
use fpt_bindings::*;
use fpt_common::*;

/// Update and save total value of assets as a point in time.
#[get("/save_pit")]
pub async fn save_pit() -> impl Responder {
    fn save_pit_to_database() -> Result<()> {
        let db = Database::open()?;
        let assets = db.get_assets()?;

        let mut total_value_usd = 0.0;
        for asset in assets {
            total_value_usd +=
                currency::get_value(&asset.code.to_lowercase(), "usd")? * asset.amount;
        }

        db.add_pit(Pit::new(total_value_usd)?)?;

        Ok(())
    }

    match save_pit_to_database() {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(error) => {
            eprint!("{error}");
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
