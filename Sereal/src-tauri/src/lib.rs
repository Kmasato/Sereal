mod adapter;
mod serial;
mod transport;
mod transport_server;

use crate::serial::service::SerialService;
use crate::transport_server::TransportServer;
use std::sync::{Arc, Mutex};

#[tauri::command]
fn connect(
    app_handle: tauri::AppHandle,
    server: tauri::State<'_, Arc<Mutex<TransportServer>>>,
    port_name: String,
    baud_rate: u32,
) {
    let server = server.lock().unwrap();
    let handler = Arc::new(adapter::TauriDataHandler::new(app_handle));
    server.connect(&port_name, baud_rate, handler);
}

#[tauri::command]
fn disconnect(server: tauri::State<'_, Arc<Mutex<TransportServer>>>, port_name: String) {
    server.lock().unwrap().disconnect(&port_name);
}

#[tauri::command]
fn get_ports(service: tauri::State<'_, Arc<Mutex<SerialService>>>) -> Vec<String> {
    let service = service.lock().unwrap();
    service.get_connectable_ports(None)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let serial_service = Arc::new(Mutex::new(SerialService::default()));
    let server = Arc::new(Mutex::new(TransportServer::new(serial_service.clone())));

    server.lock().unwrap().main();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(serial_service)
        .manage(server)
        .invoke_handler(tauri::generate_handler![get_ports, connect, disconnect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
