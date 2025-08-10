mod serial;
use eframe::egui;

pub struct MyApp {
    serial_controller: serial::Controller,
    received_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            serial_controller: serial::Controller::default(),
            received_text: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // シリアルの受信処理
        if let Some(receiver) = &self.serial_controller.receiver {
            for text in receiver.try_iter() {
                self.received_text.push_str(&text);
            }
        }

        egui::TopBottomPanel::top("").show(&ctx, |ui| {
            // SerialPort を選択する ComboBox を用意
            let port_combo_box = egui::ComboBox::from_id_salt("SerialPort")
                .selected_text(self.serial_controller.port_name());

            // BaudRate を選択する ComboBox を用意
            let baud_rate_combo_box = egui::ComboBox::from_id_salt("BaudRate")
                .selected_text(std::format!("{}", self.serial_controller.baud_rate()));

            ui.horizontal(|ui| {
                // SerialPort を選択する ComboBox の描画
                port_combo_box.show_ui(ui, |ui| {
                    let available_ports = serial::utils::list_serial_port();
                    if available_ports.is_empty() {
                        ui.label("No Serial Ports found.");
                    } else {
                        for port in &available_ports {
                            ui.selectable_value(
                                self.serial_controller.port_name_mut(),
                                port.clone(),
                                port.clone(),
                            );
                        }
                    }
                });

                // BaudRate を選択する ComboBox の描画
                baud_rate_combo_box.show_ui(ui, |ui| {
                    for rate in serial::BaudRate::iter() {
                        ui.selectable_value(
                            self.serial_controller.baud_rate_mut(),
                            rate,
                            format!("{}", rate),
                        );
                    }
                });

                // 接続ボタン
                ui.scope(|ui| {
                    // 接続ボタンのテキスト
                    let connect_button_text = if self.serial_controller.is_connect() {
                        "Connected"
                    } else {
                        "DisConnected"
                    };

                    // 接続ボタンの色
                    let connect_button_color = if self.serial_controller.is_connect() {
                        egui::Color32::from_rgb(0, 150, 0)
                    } else {
                        egui::Color32::from_rgb(200, 0, 0)
                    };

                    ui.visuals_mut().widgets.inactive.weak_bg_fill = connect_button_color;
                    if ui.button(connect_button_text).clicked() {
                        if !self.serial_controller.is_connect() {
                            // 接続処理
                            match self.serial_controller.connect() {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Error:{e}");
                                }
                            }
                        } else {
                            // 切断処理
                            self.serial_controller.disconnect();
                        }
                    };
                });
            });
        });

        // 受診結果の表示
        egui::CentralPanel::default().show(&ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.add(egui::Label::new(&self.received_text).wrap());
                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
            })
        });

        // 描画を継続的に更新するようにリクエスト
        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_title("Sereal"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Sereal",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    );
}
