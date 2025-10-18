use super::{controller::Controller, utils};
use std::collections::HashMap;

#[derive(Default)]
pub struct SerialService {
    controllers: HashMap<String, Controller>,
}

impl SerialService {
    pub fn connect(
        &mut self,
        port_name: &str,
        baud_rate: super::types::BaudRate,
    ) -> Result<(), serialport::Error> {
        if self.controllers.contains_key(port_name) {
            return Ok(());
        }

        let mut controller = Controller::default();
        *controller.port_name_mut() = port_name.to_string();
        *controller.baud_rate_mut() = baud_rate;

        match controller.connect() {
            Ok(_) => {
                self.controllers.insert(port_name.to_string(), controller);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn disconnect(&mut self, port_name: &str) {
        if let Some(mut controller) = self.controllers.remove(port_name) {
            controller.disconnect();
        }
    }

    pub fn is_connected(&self, port_name: &str) -> bool {
        self.controllers.contains_key(port_name)
    }

    pub fn get_controller(&self, port_name: &str) -> Option<&Controller> {
        self.controllers.get(port_name)
    }

    pub fn get_available_ports(&self, self_port_name: Option<&str>) -> Vec<String> {
        let all_ports = utils::list_serial_port();
        all_ports
            .into_iter()
            .filter(|port| {
                if let Some(self_port) = self_port_name {
                    if self_port == port {
                        return true;
                    }
                }
                // 接続済みのポートはリストから除外する
                !self.is_connected(port)
            })
            .collect()
    }
}
