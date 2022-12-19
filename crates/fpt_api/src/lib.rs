mod database;
mod paths;
mod tests;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use anyhow::Result;
use database::Database;

/// Start the fpt_api.
pub async fn start() -> Result<()> {
    fpt_common::Config::load()?;
    Database::open()?;

    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(paths::get_assets)
            .service(paths::update_assets)
            .service(paths::update_values)
            .service(paths::get_pits)
            .service(paths::save_pit)
    })
    .bind(("localhost", 5050))?
    .run()
    .await?;

    Ok(())
}
