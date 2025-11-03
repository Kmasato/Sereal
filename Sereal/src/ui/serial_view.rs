use std::sync::Arc;

use crate::ansi_formatter;
use crate::sereal_colors;
use crate::serial;
use crate::serial::BaudRate;
use eframe::egui;

const HISTORY_MAX_LINES: usize = 5000;

#[derive(Default)]
pub struct SerialView {
    serial_service: Arc<std::sync::Mutex<serial::service::SerialService>>,
    port_name: String,
    baud_rate: serial::BaudRate,
    received_text: String,
    received_line_count: usize,
    formatter: ansi_formatter::AnsiFormatter,
    is_autoscroll_enabled: bool,
}

impl Drop for SerialView {
    fn drop(&mut self) {
        let mut service = self.serial_service.lock().unwrap();
        service.disconnect(&self.port_name);
    }
}

impl SerialView {
    pub fn new(
        port_name: String,
        serial_service: Arc<std::sync::Mutex<serial::service::SerialService>>,
    ) -> Self {
        Self {
            serial_service,
            port_name,
            baud_rate: serial::BaudRate::default(),
            received_text: String::new(),
            received_line_count: 0,
            formatter: ansi_formatter::AnsiFormatter::default(),
            is_autoscroll_enabled: true,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        // シリアルの受信処理
        {
            let service = self.serial_service.lock().unwrap();
            if let Some(controller) = service.get_controller(&self.port_name) {
                if let Some(receiver) = &controller.receiver {
                    for text in receiver.try_iter() {
                        self.received_text.push_str(&text);
                        if text.contains('\n') {
                            self.received_line_count += 1;
                        }
                    }
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
            let available_ports = {
                let service = self.serial_service.lock().unwrap();
                service.get_available_ports(Some(&self.port_name))
            };

            // NOTE:#56 ComboBoxがリサイズしない不具合のWA
            let combo_box_id =
                ui.make_persistent_id(format!("port_combo_box_with_{}", available_ports.len()));
            let port_combo_box =
                egui::ComboBox::from_id_salt(combo_box_id).selected_text(self.port_name.clone());

            ui.horizontal(|ui| {
                // SerialPort を選択する ComboBox の描画
                port_combo_box.show_ui(ui, |ui| {
                    if available_ports.is_empty() {
                        ui.label("No Serial Ports found.");
                    } else {
                        let last_port_name = self.port_name.clone();
                        for port in &available_ports {
                            if ui
                                .selectable_value(&mut self.port_name, port.clone(), port.clone())
                                .changed()
                            {
                                self.disconnect_and_connect(
                                    &last_port_name,
                                    &self.port_name,
                                    self.baud_rate,
                                );
                            }
                        }
                    }
                });

                // BaudRate を選択する ComboBox を用意
                let baud_rate_combo_box = egui::ComboBox::from_id_salt("BaudRate")
                    .selected_text(std::format!("{}", self.baud_rate));

                // BaudRate を選択する ComboBox の描画
                baud_rate_combo_box.show_ui(ui, |ui| {
                    for rate in serial::BaudRate::iter() {
                        if ui
                            .selectable_value(&mut self.baud_rate, rate, format!("{}", rate))
                            .changed()
                        {
                            self.disconnect_and_connect(
                                &self.port_name,
                                &self.port_name,
                                self.baud_rate,
                            );
                        }
                    }
                });

                // 接続ボタン
                {
                    let is_physical_connected = {
                        let service = self.serial_service.lock().unwrap();
                        service.is_physical_connected(&self.port_name)
                    };

                    let is_connected = {
                        let service = self.serial_service.lock().unwrap();
                        service.is_connected(&self.port_name)
                    };

                    const CONNECT_BUTTON_SIZE: egui::Vec2 = egui::Vec2 { x: 18.0, y: 18.0 };
                    let connect_icon = if is_connected && !is_physical_connected {
                        egui::Image::new(egui::include_image!("../../assets/disconnect.svg"))
                            .fit_to_exact_size(CONNECT_BUTTON_SIZE)
                            .tint(sereal_colors::UI_WHITE.to_egui_color32())
                    } else {
                        egui::Image::new(egui::include_image!("../../assets/connect.svg"))
                            .fit_to_exact_size(CONNECT_BUTTON_SIZE)
                            .tint(if is_connected {
                                sereal_colors::UI_WHITE.to_egui_color32()
                            } else {
                                ui.visuals().text_color()
                            })
                    };

                    let connect_button = egui::Button::image(connect_icon).fill(if is_connected {
                        if is_physical_connected {
                            sereal_colors::UI_GREEN.to_egui_color32()
                        } else {
                            sereal_colors::UI_RED.to_egui_color32()
                        }
                    } else {
                        ui.visuals().code_bg_color
                    });

                    if ui
                        .add(connect_button)
                        .on_hover_text(if is_connected {
                            "Disconnect"
                        } else {
                            "Connect"
                        })
                        .clicked()
                    {
                        let mut service = self.serial_service.lock().unwrap();
                        if !is_connected {
                            // 接続処理
                            match service.connect(&self.port_name, self.baud_rate) {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("Error:{e}");
                                }
                            }
                        } else {
                            // 切断処理
                            service.disconnect(&self.port_name);
                        }
                    }
                }
                // クリアボタン
                const ERASER_BUTTON_SIZE: egui::Vec2 = egui::Vec2 { x: 15.0, y: 15.0 };
                let clear_button = egui::Button::image(
                    egui::Image::new(egui::include_image!("../../assets/eraser.svg"))
                        .fit_to_exact_size(ERASER_BUTTON_SIZE)
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
        self.port_name.to_string()
    }

    fn disconnect_and_connect(
        &self,
        disconnect_port_name: &str,
        connect_port_name: &str,
        connect_baud_rate: BaudRate,
    ) {
        let mut service = self.serial_service.lock().unwrap();

        service.disconnect(disconnect_port_name);
        match service.connect(connect_port_name, connect_baud_rate) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error:{e}");
            }
        }
    }
}
