use crate::serial;
use crate::serial::service::SerialService;
use crate::serial::BaudRate;
use crate::transport::DataUpdateHandler;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct SessionClient {
    port_name: String,
    last_received_id: u64,
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

    pub fn connect(&self, port_name: &String, baud_rate: u32, handler: Arc<dyn DataUpdateHandler>) {
        self.add_client(port_name.clone(), handler);
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

    fn add_client(&self, port_name: String, handler: Arc<dyn DataUpdateHandler>) {
        let mut clients = self.clients.lock().unwrap();
        clients.push(SessionClient {
            port_name: port_name,
            last_received_id: 0,
            handler: handler,
        });
    }
}
