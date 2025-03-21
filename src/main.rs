use std::error::Error;

mod computer;
mod notifications;
mod websocket;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let mut socket = websocket::connect().expect("Could not connect to WebSocket");
    computer::start_sending(&mut socket);

    Ok(())
}
