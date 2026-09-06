use crate::transport::DataUpdateHandler;
use tauri::Emitter;

pub struct TauriDataHandler {
    app_handle: tauri::AppHandle,
    client_id: String,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct SerialEventPayload {
    client_id: String,
    text: String,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ConnectionStatusPayload {
    client_id: String,
    status: crate::serial::types::ConnectionStatus,
}

impl TauriDataHandler {
    pub fn new(app_handle: tauri::AppHandle, client_id: String) -> Self {
        Self {
            app_handle,
            client_id,
        }
    }
}

impl DataUpdateHandler for TauriDataHandler {
    fn on_received(&self, data: Vec<u8>) {
        let text = String::from_utf8_lossy(&data).to_string();
        let payload = SerialEventPayload {
            client_id: self.client_id.clone(),
            text,
        };
        let _ = self.app_handle.emit("serial-data", payload);
    }

    fn on_error(&self, message: String) {
        let _ = self.app_handle.emit("serial-error", message);
    }

    fn on_status_changed(&self, status: crate::serial::types::ConnectionStatus) {
        let payload = ConnectionStatusPayload {
            client_id: self.client_id.clone(),
            status: status,
        };
        let _ = self.app_handle.emit("connection-status-changed", payload);
    }
}
