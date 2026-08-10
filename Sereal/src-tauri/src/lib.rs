mod adapter;
mod serial;
mod transport;
mod transport_server;

use crate::serial::service::SerialService;
use crate::transport_server::TransportServer;
use std::sync::{Arc, Mutex};

#[tauri::command]
fn register_handler(
    app_handle: tauri::AppHandle,
    server: tauri::State<'_, Arc<Mutex<TransportServer>>>,
    client_id: String,
) {
    let handler = Arc::new(adapter::TauriDataHandler::new(
        app_handle,
        client_id.clone(),
    ));
    server.lock().unwrap().register_handler(client_id, handler);
}

#[tauri::command]
fn unregister_handler(server: tauri::State<'_, Arc<Mutex<TransportServer>>>, client_id: String) {
    server.lock().unwrap().unregister_handler(client_id);
}

#[tauri::command]
fn connect(
    server: tauri::State<'_, Arc<Mutex<TransportServer>>>,
    client_id: String,
    port_name: String,
    baud_rate: u32,
) -> bool {
    server
        .lock()
        .unwrap()
        .connect(client_id, port_name, baud_rate)
}

#[tauri::command]
fn disconnect(server: tauri::State<'_, Arc<Mutex<TransportServer>>>, client_id: String) -> bool {
    server.lock().unwrap().disconnect(client_id)
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
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                use tauri::Manager;

                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register_handler,
            unregister_handler,
            connect,
            disconnect,
            get_ports,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
