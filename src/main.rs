use std::error::Error;

mod websocket;
mod notifications;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let mut socket = websocket::connect().unwrap();
    websocket::send(&mut socket, 1, 1);

    Ok(())
}
