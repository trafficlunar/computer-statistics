use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use futures_util::{SinkExt, StreamExt};
use inputbot::{KeybdKey, MouseButton};
use sysinfo::System;
use tokio::{net::TcpStream, time::timeout};
use tokio_tungstenite::{
    tungstenite::{Bytes, Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::websocket;

pub async fn start_sending(socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) {
    let mut sys = System::new();

    let key_counter = Arc::new(AtomicU16::new(0));
    let click_counter = Arc::new(AtomicU16::new(0));

    let key_counter_clone = Arc::clone(&key_counter);
    let click_counter_clone = Arc::clone(&click_counter);

    // Keys and clicks handler
    tokio::task::spawn_blocking(move || {
        KeybdKey::bind_all(move |_| {
            key_counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        MouseButton::bind_all(move |event| match event {
            MouseButton::LeftButton | MouseButton::RightButton => {
                click_counter_clone.fetch_add(1, Ordering::SeqCst);
            }
            _ => {}
        });

        inputbot::handle_input_events();
    });

    let mut last_ping_sent = Instant::now() - Duration::from_secs(30);
    let mut last_stats = Instant::now() - Duration::from_secs(60);

    loop {
        let now = Instant::now();

        // Send ping every 30 seconds
        if now.duration_since(last_ping_sent) >= Duration::from_secs(30) {
            last_ping_sent = now;

            println!("Sending ping...");
            if let Err(e) = socket.send(Message::Ping(Bytes::new())).await {
                eprintln!("Failed to send ping: {}", e);
                break;
            }

            // Read incoming messages
            match timeout(Duration::from_secs(10), socket.next()).await {
                Ok(Some(msg)) => match msg {
                    Ok(Message::Pong(_)) => {
                        println!("Received pong");
                    }
                    Ok(Message::Close(_)) => {
                        eprintln!("Received close, reconnecting...");
                        break;
                    }
                    Ok(_) => {} // other messages
                    Err(e) => {
                        eprintln!("Error receiving message: {}, reconnecting...", e);
                        break;
                    }
                },
                Ok(None) => {
                    // Stream ended
                    eprintln!("WebSocket stream ended, reconnecting...");
                    break;
                }
                Err(_) => {
                    // Timed out waiting for message
                    eprintln!("No response received in 10 seconds, reconnecting...");
                    break;
                }
            }
        }

        // Send stats every 60 seconds
        if now.duration_since(last_stats) >= Duration::from_secs(60) {
            last_stats = now;

            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let cpu_usage = sys.global_cpu_usage().floor() as u8;
            let total_memory = sys.total_memory();
            let used_memory = sys.used_memory();
            let memory_usage = ((used_memory as f64) / (total_memory as f64) * 100.0).floor() as u8;

            let keys = key_counter.load(Ordering::SeqCst);
            let clicks = click_counter.load(Ordering::SeqCst);

            if let Err(e) = websocket::send(socket, cpu_usage, memory_usage, keys, clicks).await {
                eprintln!("Failed to send statistics: {}", e);
                break;
            }

            key_counter.store(0, Ordering::SeqCst);
            click_counter.store(0, Ordering::SeqCst);
        }

        thread::sleep(Duration::from_secs(1));
    }
}
