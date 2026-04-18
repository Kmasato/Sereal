mod serial;

use crate::serial::service::SerialService;
use std::sync::Mutex;

#[tauri::command]
fn get_ports(service: tauri::State<'_, Mutex<SerialService>>) -> Vec<String> {
    let service = service.lock().unwrap();
    service.get_connectable_ports(None)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(SerialService::default()))
        .invoke_handler(tauri::generate_handler![get_ports])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
