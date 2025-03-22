use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use inputbot::{KeybdKey, MouseButton};
use sysinfo::System;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

use crate::websocket;

pub fn start_sending(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let mut sys = System::new();

    let key_counter = Arc::new(Mutex::new(0_u16));
    let click_counter = Arc::new(Mutex::new(0_u16));

    let key_counter_clone = Arc::clone(&key_counter);
    let click_counter_clone = Arc::clone(&click_counter);

    thread::spawn(move || {
        KeybdKey::bind_all(move |_| {
            let mut count = key_counter_clone.lock().unwrap();
            *count += 1;
        });

        MouseButton::bind_all(move |event| match event {
            MouseButton::LeftButton | MouseButton::RightButton => {
                let mut count = click_counter_clone.lock().unwrap();
                *count += 1;
            }
            _ => {}
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

        let mut key_guard = key_counter.lock().unwrap();
        let key_presses = *key_guard;

        let mut click_guard = click_counter.lock().unwrap();
        let clicks = *click_guard;

        websocket::send(socket, cpu_usage, memory_usage, key_presses, clicks);

        // Reset counters after sending
        *key_guard = 0;
        *click_guard = 0;

        thread::sleep(Duration::from_secs(60));
    }
}
