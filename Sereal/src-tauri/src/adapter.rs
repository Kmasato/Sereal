use crate::transport::DataUpdateHandler;
use tauri::Emitter;

pub struct TauriDataHandler {
    app_handle: tauri::AppHandle,
}

impl TauriDataHandler {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }
}

impl DataUpdateHandler for TauriDataHandler {
    fn on_received(&self, data: Vec<u8>) {
        let text = String::from_utf8_lossy(&data).to_string();
        let _ = self.app_handle.emit("serial-data", text);
    }

    fn on_error(&self, message: String) {
        let _ = self.app_handle.emit("serial-error", message);
    }
}
