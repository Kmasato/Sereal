use serialport;
use std::println;

pub fn list_serial_port() -> Vec<String> {
    let mut out_ports = Vec::new();
    if let Ok(physhical_ports) = serialport::available_ports() {
        for p in physhical_ports {
            out_ports.push(p.port_name);
        }
    }

    // Virtual Device 向けの仮想ポートを列挙
    #[cfg(unix)]
    {
        use std::fs;
        if let Ok(entries) = fs::read_dir("/tmp") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    if path_str.starts_with("/tmp/vd_port") {
                        if !out_ports.contains(&path_str.to_string()) {
                            out_ports.push(path_str.to_string());
                        }
                    }
                }
            }
        }
    }

    if out_ports.is_empty() {
        println!("No serial ports found.");
    }

    out_ports
}
