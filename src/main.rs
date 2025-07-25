use std::{error::Error, thread, time::Duration};

mod computer;
mod notifications;
mod websocket;

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    loop {
        match websocket::connect() {
            Ok(mut socket) => {
                println!("WebSocket connected successfully");

                // This will block until connection fails
                computer::start_sending(&mut socket);

                // The connection failed if code has reached here
                println!("WebSocket connection lost, attempting to reconnect in 10 seconds...");
            }
            Err(_) => {
                println!("Retrying connection in 10 seconds...");
            }
        }

        // Wait 5 seconds before attempting to reconnect
        thread::sleep(Duration::from_secs(5));
    }
}
