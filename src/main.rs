use std::error::Error;

mod websocket;
mod computer;
mod notifications;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let mut socket = websocket::connect().unwrap();
    computer::start_sending(&mut socket);

    Ok(())
}
