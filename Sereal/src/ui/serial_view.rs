use crate::ansi_formatter;
use crate::sereal_colors;
use crate::serial;
use eframe::egui;

const HISTORY_MAX_LINES: usize = 5000;

pub struct SerialView {
    id: usize,
    serial_controller: serial::Controller,
    received_text: String,
    received_line_count: usize,
    formatter: ansi_formatter::AnsiFormatter,
    is_autoscroll_enabled: bool,
}

impl Default for SerialView {
    fn default() -> Self {
        Self {
            id: 0,
            serial_controller: serial::Controller::default(),
            received_text: String::new(),
            received_line_count: 0,
            formatter: ansi_formatter::AnsiFormatter::default(),
            is_autoscroll_enabled: true,
        }
    }
}

impl SerialView {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            serial_controller: serial::Controller::default(),
            received_text: String::new(),
            received_line_count: 0,
            formatter: ansi_formatter::AnsiFormatter::default(),
            is_autoscroll_enabled: true,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // シリアルの受信処理
        if let Some(receiver) = &self.serial_controller.receiver {
            for text in receiver.try_iter() {
                self.received_text.push_str(&text);
                if text.contains('\n') {
                    self.received_line_count += 1;
                }
            }
        }

        // FIXME:単純に削ると以前のデザイン情報が削られるため直す必要あり
        if HISTORY_MAX_LINES < self.received_line_count {
            if let Some(index) = self.received_text.find('\n') {
                self.received_text.drain(..=index);
                self.received_line_count -= 1;
            }
        }

        ui.vertical(|ui| {
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
                let is_connect = self.serial_controller.is_connect();

                let connect_icon =
                    egui::Image::new(egui::include_image!("../../assets/connect.svg"))
                        .fit_to_exact_size(egui::Vec2 { x: 20.0, y: 20.0 })
                        .tint(if is_connect {
                            sereal_colors::UI_WHITE.to_egui_color32()
                        } else {
                            ui.visuals().text_color()
                        });

                let connect_button = egui::Button::image(connect_icon).fill(if is_connect {
                    sereal_colors::UI_GREEN.to_egui_color32()
                } else {
                    ui.visuals().code_bg_color
                });

                if ui
                    .add(connect_button)
                    .on_hover_text(if is_connect { "Disconnect" } else { "Connect" })
                    .clicked()
                {
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
                }

                // クリアボタン
                let clear_button = egui::Button::image(
                    egui::Image::new(egui::include_image!("../../assets/eraser.svg"))
                        .fit_to_exact_size(egui::Vec2 { x: 15.0, y: 15.0 })
                        .tint(ui.visuals().text_color()),
                );
                if ui
                    .add(clear_button)
                    .on_hover_text("Clear all buffer")
                    .clicked()
                {
                    self.formatter.reset();
                    self.received_text.clear();
                    self.received_line_count = 0;
                }
            });
        });

        // コントロール部と表示部の区切り線
        ui.separator();

        egui::ScrollArea::vertical()
            .stick_to_bottom(self.is_autoscroll_enabled)
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing = egui::Vec2 { x: 0.0, y: 0.0 };

                    // FIXME:毎回変換を行っているが、処理負荷が重いため修正する
                    for line in self.received_text.lines() {
                        ui.horizontal_wrapped(|ui| {
                            for rich_text in self.formatter.to_rich_text(&line.to_string()) {
                                ui.label(rich_text);
                            }
                        });
                    }
                });
                self.formatter.reset();
            });
    }

    pub fn get_port_name(&self) -> String {
        let port_name = self.serial_controller.get_port_name();
        match port_name.as_str() {
            "" => {
                format!("Port {}", self.id)
            }
            _name => port_name,
        }
    }
}
