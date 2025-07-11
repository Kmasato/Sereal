use core::fmt;
use eframe::egui;
use serialport;

#[derive(Debug, PartialEq, Clone, Copy)]
enum BaudRate {
    BaudRate9600 = 9600,
    BaudRate115200 = 115200,
}

impl BaudRate {
    fn iter() -> impl Iterator<Item = BaudRate> {
        [BaudRate::BaudRate9600, BaudRate::BaudRate115200]
            .iter()
            .copied()
    }
}

impl fmt::Display for BaudRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rate = match self {
            BaudRate::BaudRate9600 => 9600,
            BaudRate::BaudRate115200 => 115200,
        };
        write!(f, "{}", rate)
    }
}

pub struct MyApp {
    port: Option<Box<dyn serialport::SerialPort>>,
    port_name: String,
    baud_rate: BaudRate,
    is_connect: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            port: None,
            port_name: "Select Serial Port".to_string(),
            baud_rate: BaudRate::BaudRate115200,
            is_connect: false,
        }
    }
}

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
        egui::TopBottomPanel::top("").show(&ctx, |ui| {
            // SerialPort を選択する ComboBox を用意
            let port_combo_box =
                egui::ComboBox::from_id_salt("SerialPort").selected_text(self.port_name.clone());

            // BaudRate を選択する ComboBox を用意
            let baud_rate_combo_box = egui::ComboBox::from_id_salt("BaudRate")
                .selected_text(std::format!("{}", self.baud_rate));

            ui.horizontal(|ui| {
                // SerialPort を選択する ComboBox の描画
                port_combo_box.show_ui(ui, |ui| {
                    let available_ports = list_serial_port();
                    if available_ports.is_empty() {
                        ui.label("No Serial Ports found.");
                    } else {
                        for port in &available_ports {
                            ui.selectable_value(&mut self.port_name, port.clone(), port.clone());
                        }
                    }
                });

                // BaudRate を選択する ComboBox の描画
                baud_rate_combo_box.show_ui(ui, |ui| {
                    for rate in BaudRate::iter() {
                        ui.selectable_value(&mut self.baud_rate, rate, format!("{}", rate));
                    }
                });

                // 接続ボタン
                ui.scope(|ui| {
                    // 接続ボタンのテキスト
                    let connect_button_text = if self.is_connect {
                        "Connected"
                    } else {
                        "DisConnected"
                    };

                    // 接続ボタンの色
                    let connect_button_color = if self.is_connect {
                        egui::Color32::from_rgb(0, 150, 0)
                    } else {
                        egui::Color32::from_rgb(200, 0, 0)
                    };

                    ui.visuals_mut().widgets.inactive.weak_bg_fill = connect_button_color;
                    if ui.button(connect_button_text).clicked() {
                        if !self.is_connect {
                            match serialport::new(self.port_name.clone(), self.baud_rate as u32)
                                .open()
                            {
                                Ok(port) => {
                                    self.port = Some(port);
                                    self.is_connect = true;
                                    println!("Connected Success!");
                                }
                                Err(e) => {
                                    eprintln!("Failed to connect {}", e);
                                }
                            }
                        } else {
                            self.port.take(); // Optional の中を取り出して None にする
                            self.is_connect = false;
                            println!("Disconnected Success!")
                        }
                    };
                });
            });
        });

        // TODO : 受信結果を表示
        egui::CentralPanel::default().show(&ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {});
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
