use serialport;

pub fn list_serial_port() -> Vec<String> {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("No serial ports found.");
                Vec::new()
            } else {
                let mut port_names = Vec::new();
                for p in ports {
                    port_names.push(p.port_name);
                }
                port_names
            }
        }
        Err(e) => {
            eprintln!("Error listing serial ports:{}", e);
            Vec::new()
        }
    }
}
