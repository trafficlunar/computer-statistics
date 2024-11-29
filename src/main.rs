use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    println!("Hello, world!");

    Ok(())
}
