mod paths;
mod tests;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use anyhow::Result;

pub async fn start() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(paths::www())
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    Ok(())
}
