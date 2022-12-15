mod paths;
mod tests;

use actix_web::{App, HttpServer};
use anyhow::Result;

pub async fn start() -> Result<()> {
    HttpServer::new(|| App::new().service(paths::www()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
