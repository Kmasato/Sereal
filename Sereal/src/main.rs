use eframe::egui;
use serialport;

#[derive(Default, Clone)]
pub struct MyApp {}

fn list_serial_port() -> Vec<String> {
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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            let available_ports = list_serial_port();
            for port in available_ports {
                ui.label(port);
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("Sereal"),
        ..Default::default()
    };

    list_serial_port();

    let _ = eframe::run_native(
        "Sereal",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    );
}
