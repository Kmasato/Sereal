use crate::serial;
use crate::serial::service::SerialService;
use crate::serial::types::ConnectionStatus;
use crate::serial::BaudRate;
use crate::transport::DataUpdateHandler;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct SessionClient {
    port_name: Option<String>,
    last_received_id: u64,
    last_connection_status: ConnectionStatus,
    handler: Arc<dyn DataUpdateHandler>,
}

pub struct TransportServer {
    serial_service: Arc<Mutex<SerialService>>,
    clients: Arc<Mutex<HashMap<String, SessionClient>>>,
}

impl TransportServer {
    pub const POLLING_INTERVAL: Duration = Duration::from_millis(16);

    pub fn new(serial_service: Arc<Mutex<SerialService>>) -> Self {
        Self {
            serial_service,
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn main(&self) {
        let serial_service = self.serial_service.clone();
        let clients = self.clients.clone();

        thread::spawn(move || loop {
            thread::sleep(TransportServer::POLLING_INTERVAL);

            let service = serial_service.lock().unwrap();
            let mut client_list = clients.lock().unwrap();

            for (_client_id, client) in client_list.iter_mut() {
                let port_name = match &client.port_name {
                    Some(name) => name,
                    None => continue, //
                };

                // 接続ステータスの更新
                let current_status = Self::get_connection_status(&service, &port_name);
                if current_status != client.last_connection_status {
                    client.last_connection_status = current_status;
                    client
                        .handler
                        .on_status_changed(client.last_connection_status.clone());
                    println!("State changed {:?}", client.last_connection_status);
                }

                // 受信データの更新
                let received_data = service
                    .get_received_data(&port_name, serial::types::MAX_RECEIVED_DATA_SIZE as u16);

                let new_data: Vec<_> = received_data
                    .into_iter()
                    .filter(|d| d.id > client.last_received_id)
                    .collect();

                if let Some(last) = new_data.last() {
                    let last_id = last.id;
                    let combined_text: String = new_data.into_iter().map(|d| d.text).collect();

                    client.handler.on_received(combined_text.into_bytes());
                    client.last_received_id = last_id;
                };
            }
        });
    }

    pub fn register_handler(&self, client_id: String, handler: Arc<dyn DataUpdateHandler>) {
        self.add_client(client_id.clone(), handler);
    }

    pub fn unregister_handler(&self, client_id: String) {
        self.remove_client(client_id);
    }

    pub fn connect(&self, client_id: String, port_name: String, baud_rate: u32) {
        if let Some(client) = self.clients.lock().unwrap().get_mut(&client_id) {
            let mut service = self.serial_service.lock().unwrap();
            if !service.is_connected(&port_name) {
                match service.connect(&port_name, BaudRate::from_u32(baud_rate)) {
                    Ok(_) => {
                        client.port_name = Some(port_name.clone());
                        println!("Connect {port_name}");
                    }
                    Err(e) => {
                        eprintln!("Connection Failed:{e}");
                    }
                };
            } else {
                println!("Physical port is already connected\n");
            }
        }
    }

    pub fn disconnect(&self, client_id: String) {
        if let Some(client) = self.clients.lock().unwrap().get_mut(&client_id) {
            if let Some(port_name) = &client.port_name {
                let mut service = self.serial_service.lock().unwrap();
                if service.is_connected(&port_name) {
                    // 接続を切断
                    service.disconnect(&port_name);
                    // 受信したレポートIDをリセット
                    client.last_received_id = 0;
                } else {
                    eprintln!("Failed to disconnect, {port_name} is not opened.")
                }
            }
        }
    }

    fn add_client(&self, client_id: String, handler: Arc<dyn DataUpdateHandler>) {
        let mut clients = self.clients.lock().unwrap();
        clients.insert(
            client_id,
            SessionClient {
                port_name: None,
                last_received_id: 0,
                last_connection_status: ConnectionStatus::Disconnected,
                handler: handler,
            },
        );
    }

    fn remove_client(&self, client_id: String) {
        self.clients.lock().unwrap().remove(&client_id);
    }

    fn get_connection_status(
        serial_service: &SerialService,
        port_name: &String,
    ) -> ConnectionStatus {
        let is_physical_connected = serial_service.is_physical_connected(port_name);
        let is_connected = serial_service.is_connected(port_name);

        if is_connected {
            if is_physical_connected {
                return ConnectionStatus::Connected;
            } else {
                return ConnectionStatus::PhysicalDisconnected;
            }
        }
        return ConnectionStatus::Disconnected;
    }
}
