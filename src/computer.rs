use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use inputbot::KeybdKey;
use sysinfo::System;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

use crate::websocket;

pub fn start_sending(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let mut sys = System::new();

    // Key counter
    let key_counter = Arc::new(Mutex::new(0_u16));
    let key_counter_clone = Arc::clone(&key_counter);
    thread::spawn(move || {
        KeybdKey::bind_all(move |_| {
            let mut count = key_counter_clone.lock().unwrap();
            *count += 1;
        });

        inputbot::handle_input_events();
    });

    loop {
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let cpu_usage = sys.global_cpu_usage().floor() as u8;

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let memory_usage = ((used_memory as f64) / (total_memory as f64) * 100.0).floor() as u8;

        let mut guard = key_counter.lock().unwrap();
        let key_presses = *guard;

        websocket::send(socket, cpu_usage, memory_usage, key_presses);

        // Reset key press counter after sending
        *guard = 0;

        thread::sleep(Duration::from_secs(60));
    }
}
