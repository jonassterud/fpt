use anyhow::Result;

fn main() -> Result<()> {
    fpt_api::start()?;
    fpt_web::start()?;

    println!("Access fpt at: 127.0.0.1:8080");

    Ok(())
}
