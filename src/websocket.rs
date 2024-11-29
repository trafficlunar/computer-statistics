use std::{env, net::TcpStream};

use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};

use crate::notifications;

pub fn connect() -> Result<WebSocket<MaybeTlsStream<TcpStream>>, tungstenite::Error> {
    let (socket, _) = match tungstenite::connect(env::var("WEBSOCKET_URL").unwrap()) {
        Ok(ws) => ws,
        Err(err) => {
            eprintln!("Unable to connect to WebSocket: {}", err);
            notifications::send_error_notification("Unable to connect to WebSocket");

            return Err(err);
        }
    };

    println!("Connected to WebSocket");
    Ok(socket)
}

pub fn send(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, cpu: u8, ram: u8) {
    let message = format!("{{ \"cpu\": {}, \"ram\": {} }}", cpu, ram);

    println!("Sending to WebSocket: {}", message);

    socket
        .send(Message::Text(message))
        .unwrap();
}
