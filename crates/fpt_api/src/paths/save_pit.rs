use crate::{structures, Database};
use actix_web::{get, Responder};
use anyhow::Result;

/// Update and save total value of assets as a point in time.
#[get("/save_pit")]
pub async fn save_pit() -> impl Responder {
    fn save_pit_to_database() -> Result<()> {
        let db = Database::open()?;
        let assets = db.get_assets()?;

        db.add_pit(structures::Pit::new(&assets)?)?;

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
