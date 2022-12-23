use actix_web::{get, web, Responder};
use anyhow::Result;

use crate::Database;

/// Get PITs from the database.
#[get("/remove_pit/{id}")]
pub async fn remove_pit(id: web::Path<usize>) -> impl Responder {
    let open_and_remove_pit = move || -> Result<()> {
        let db = Database::open()?;

        db.remove_pit(*id)?;

        Ok(())
    };

    match open_and_remove_pit() {
        Ok(_) => actix_web::HttpResponse::Ok().finish(),
        Err(error) => {
            log::error!("{error}");
            actix_web::HttpResponse::InternalServerError().finish()
        }
    }
}
