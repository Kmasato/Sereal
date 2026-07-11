use crate::transport::DataUpdateHandler;
use tauri::Emitter;

pub struct TauriDataHandler {
    app_handle: tauri::AppHandle,
    port_name: String,
}

#[derive(Clone, serde::Serialize)]
struct SerialEventPayload {
    port_name: String,
    text: String,
}

#[derive(Clone, serde::Serialize)]
struct ConnectionStatusPayload {
    port_name: String,
    status: crate::serial::types::ConnectionStatus,
}

impl TauriDataHandler {
    pub fn new(app_handle: tauri::AppHandle, port_name: String) -> Self {
        Self {
            app_handle,
            port_name,
        }
    }
}

impl DataUpdateHandler for TauriDataHandler {
    fn on_received(&self, data: Vec<u8>) {
        let text = String::from_utf8_lossy(&data).to_string();
        let payload = SerialEventPayload {
            port_name: self.port_name.clone(),
            text,
        };
        let _ = self.app_handle.emit("serial-data", payload);
    }

    fn on_error(&self, message: String) {
        let _ = self.app_handle.emit("serial-error", message);
    }

    fn on_status_changed(&self, status: crate::serial::types::ConnectionStatus) {
        let payload = ConnectionStatusPayload {
            port_name: self.port_name.clone(),
            status: status,
        };
        let _ = self.app_handle.emit("connection-status-changed", payload);
    }
}
