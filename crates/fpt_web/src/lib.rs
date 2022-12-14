mod serve;
mod tests;

use actix_web::{App, HttpServer};
use anyhow::Result;

pub async fn start() -> Result<()> {
    HttpServer::new(|| App::new())
        .bind(("http://127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
