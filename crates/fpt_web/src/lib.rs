mod paths;
mod tests;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use anyhow::Result;

pub async fn start() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new().wrap(cors).service(paths::www())
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    Ok(())
}
