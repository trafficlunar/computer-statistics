use std::{
    net::TcpStream,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use inputbot::{KeybdKey, MouseButton};
use sysinfo::System;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

use crate::websocket;

pub fn start_sending(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let mut sys = System::new();

    let key_counter = Arc::new(AtomicU16::new(0));
    let click_counter = Arc::new(AtomicU16::new(0));

    let key_counter_clone = Arc::clone(&key_counter);
    let click_counter_clone = Arc::clone(&click_counter);

    // Keys and clicks handler
    thread::spawn(move || {
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

    // Send to WebSocket every 60 seconds
    loop {
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let cpu_usage = sys.global_cpu_usage().floor() as u8;

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let memory_usage = ((used_memory as f64) / (total_memory as f64) * 100.0).floor() as u8;

        let keys = key_counter.load(Ordering::SeqCst);
        let clicks = click_counter.load(Ordering::SeqCst);

        match websocket::send(socket, cpu_usage, memory_usage, keys, clicks) {
            Ok(_) => {
                // Reset counters after sending
                key_counter.store(0, Ordering::SeqCst);
                click_counter.store(0, Ordering::SeqCst);
            }
            Err(e) => {
                eprintln!("Failed to send WebSocket message: {}", e);
                // Avoid resetting counters because we'll try to resend them after reconnection
                break; // triggers reconnection in main.rs
            }
        }

        // Reset counters after sending
        key_counter.store(0, Ordering::SeqCst);
        click_counter.store(0, Ordering::SeqCst);

        thread::sleep(Duration::from_secs(60));
    }
}
