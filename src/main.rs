use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Access fpt at: http://127.0.0.1:8080");

    tokio::try_join!(fpt_api::start(), fpt_web::start())?;

    Ok(())
}
