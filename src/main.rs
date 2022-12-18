use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

    println!("Access fpt_api at: http://127.0.0.1:5050");
    println!("Access fpt_web at: http://127.0.0.1:8080");

    tokio::try_join!(fpt_api::start(), fpt_web::start())?;

    Ok(())
}
