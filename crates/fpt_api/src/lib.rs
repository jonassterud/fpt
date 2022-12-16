mod database;
mod paths;
mod sparebank1_api;
mod structures;
mod tests;

use actix_web::{App, HttpServer};
use anyhow::Result;
use database::Database;

pub async fn start() -> Result<()> {
    dotenv::dotenv().ok();

    Database::open()?;

    HttpServer::new(|| {
        App::new()
            .service(paths::get_assets)
            .service(paths::update_assets)
    })
    .bind(("127.0.0.1", 5050))?
    .run()
    .await?;

    Ok(())
}
