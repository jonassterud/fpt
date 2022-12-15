mod database;
mod paths;
mod structures;
mod tests;

use actix_web::{App, HttpServer};
use anyhow::Result;
use database::Database;

pub async fn start() -> Result<()> {
    dotenv::dotenv()?;

    let mut db = Database::open()?;
    db.close();

    HttpServer::new(|| App::new().service(paths::get_assets))
        .bind(("127.0.0.1", 5050))?
        .run()
        .await?;

    Ok(())
}
