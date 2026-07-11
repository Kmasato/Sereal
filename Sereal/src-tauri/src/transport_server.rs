use crate::serial;
use crate::serial::service::SerialService;
use crate::serial::types::ConnectionStatus;
use crate::serial::BaudRate;
use crate::transport::DataUpdateHandler;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct SessionClient {
    port_name: String,
    last_received_id: u64,
    last_connection_status: ConnectionStatus,
    handler: Arc<dyn DataUpdateHandler>,
}

pub struct TransportServer {
    serial_service: Arc<Mutex<SerialService>>,
    clients: Arc<Mutex<Vec<SessionClient>>>,
}

impl TransportServer {
    pub const POLLING_INTERVAL: Duration = Duration::from_millis(16);

    pub fn new(serial_service: Arc<Mutex<SerialService>>) -> Self {
        Self {
            serial_service,
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn main(&self) {
        let serial_service = self.serial_service.clone();
        let clients = self.clients.clone();

        thread::spawn(move || loop {
            thread::sleep(TransportServer::POLLING_INTERVAL);

            let service = serial_service.lock().unwrap();
            let mut client_list = clients.lock().unwrap();

            for client in client_list.iter_mut() {
                // 接続ステータスの更新
                let current_status = Self::get_connection_status(&service, &client.port_name);
                if current_status != client.last_connection_status {
                    client.last_connection_status = current_status;
                    client
                        .handler
                        .on_status_changed(client.last_connection_status.clone());
                    println!("State changed {:?}", client.last_connection_status);
                }

                // 受信データの更新
                let received_data = service.get_received_data(
                    &client.port_name,
                    serial::types::MAX_RECEIVED_DATA_SIZE as u16,
                );

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

    pub fn register_handler(&self, port_name: &String, handler: Arc<dyn DataUpdateHandler>) {
        self.add_client(port_name.clone(), handler);
    }

    pub fn unregister_handler(&self, port_name: &String) {
        self.remove_client(port_name);
    }

    pub fn connect(&self, port_name: &String, baud_rate: u32) {
        let mut service = self.serial_service.lock().unwrap();
        if !service.is_connected(&port_name) {
            match service.connect(&port_name, BaudRate::from_u32(baud_rate)) {
                Ok(_) => {
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

    pub fn disconnect(&self, port_name: &String) {
        let mut service = self.serial_service.lock().unwrap();
        if service.is_connected(&port_name) {
            // 接続を切断
            service.disconnect(&port_name);

            // 受信したレポートIDをリセット
            let mut clients = self.clients.lock().unwrap();
            if let Some(client) = clients.iter_mut().find(|c| &c.port_name == port_name) {
                client.last_received_id = 0;
            }
        } else {
            eprintln!("Failed to disconnect, because not find {port_name}.")
        }
    }

    fn add_client(&self, port_name: String, handler: Arc<dyn DataUpdateHandler>) {
        let mut clients = self.clients.lock().unwrap();
        clients.push(SessionClient {
            port_name: port_name,
            last_received_id: 0,
            last_connection_status: ConnectionStatus::Disconnected,
            handler: handler,
        });
    }

    fn remove_client(&self, port_name: &String) {
        let mut clients = self.clients.lock().unwrap();
        clients.retain(|client| &client.port_name != port_name);
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
