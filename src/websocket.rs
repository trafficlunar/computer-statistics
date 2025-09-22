use std::env;

use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::generate_key, http::Request, Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::notifications;

pub async fn connect(
) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Error> {
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

    let (socket, _) = match connect_async(request).await {
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

pub async fn send(
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    cpu: u8,
    ram: u8,
    keys: u16,
    clicks: u16,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let message = format!(
        "{{ \"cpu\": {}, \"ram\": {}, \"keys\": {}, \"clicks\": {} }}",
        cpu, ram, keys, clicks
    );

    println!("Sending to WebSocket: {}", message);

    socket.send(Message::Text(message.into())).await
}
