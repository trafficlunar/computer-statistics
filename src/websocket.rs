use std::{env, net::TcpStream};

use tungstenite::{
    handshake::client::generate_key, http::Request, stream::MaybeTlsStream, Message, WebSocket,
};
use url::Url;

use crate::notifications;

pub fn connect() -> Result<WebSocket<MaybeTlsStream<TcpStream>>, tungstenite::Error> {
    let websocket_url =
        Url::parse(&env::var("WEBSOCKET_URL").unwrap()).expect("Invalid WebSocket URL");
    let host = websocket_url.host_str().expect("Host not found in URL");

    let host_header = match websocket_url.port() {
        Some(port) => format!("{}:{}", host, port),
        None => host.to_string(),
    };

    let request = Request::builder()
        .method("GET")
        .uri(env::var("WEBSOCKET_URL").unwrap())
        .header("Authorization", env::var("WEBSOCKET_PASSWORD").unwrap())
        .header("Host", host_header)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .body(())
        .unwrap();

    let (socket, _) = match tungstenite::connect(request) {
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

pub fn send(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    cpu: u8,
    ram: u8,
    key_presses: u16,
    clicks: u16,
) {
    let message = format!(
        "{{ \"cpu\": {}, \"ram\": {}, \"key_presses\": {}, \"clicks\": {} }}",
        cpu, ram, key_presses, clicks
    );

    println!("Sending to WebSocket: {}", message);

    socket.send(Message::Text(message)).unwrap();
}
