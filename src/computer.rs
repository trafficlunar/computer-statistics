use std::{net::TcpStream, thread, time::Duration};

use sysinfo::System;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

use crate::websocket;

pub fn start_sending(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let mut sys = System::new();

    loop {
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let cpu_usage = sys.global_cpu_usage().floor() as u8;

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();

        let memory_usage = ((used_memory as f64) / (total_memory as f64) * 100.0).floor() as u8;

        websocket::send(socket, cpu_usage, memory_usage);
        thread::sleep(Duration::from_secs(60));
    }
}