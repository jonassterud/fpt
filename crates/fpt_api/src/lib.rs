mod tests;

use actix_web::{App, HttpServer};
use anyhow::Result;

pub async fn start() -> Result<()> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 5050))?
        .run()
        .await?;

    Ok(())
}
