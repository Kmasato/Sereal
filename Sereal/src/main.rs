#![windows_subsystem = "windows"]

mod ansi_formatter;
mod sereal_colors;
mod serial;
mod ui;

use eframe::egui;
use egui_dock::{DockArea, DockState, Style, TabAddAlign, TabViewer, tab_viewer::OnCloseResponse};
use std::sync::Arc;

use crate::serial::service::SerialService;

pub struct AppTabViewer<'a> {
    add_nodes: &'a mut Vec<(egui_dock::SurfaceIndex, egui_dock::NodeIndex)>,
}

impl TabViewer for AppTabViewer<'_> {
    type Tab = ui::SerialView;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.get_port_name().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.ui(ui);
    }

    fn on_close(&mut self, _tab: &mut Self::Tab) -> OnCloseResponse {
        OnCloseResponse::Close
    }

    fn on_add(&mut self, surface: egui_dock::SurfaceIndex, node: egui_dock::NodeIndex) {
        self.add_nodes.push((surface, node));
    }
}

#[derive(PartialEq, Default)]
enum Theme {
    System,
    LightMode,
    #[default]
    DarkMode,
}

pub struct MyApp {
    dock_state: DockState<ui::SerialView>,
    counter: usize,
    serial_service: Arc<std::sync::Mutex<SerialService>>,
    theme: Theme,
}

impl Default for MyApp {
    fn default() -> Self {
        let serial_service = Arc::new(std::sync::Mutex::new(SerialService::default()));
        let initial_tab = ui::SerialView::new("".to_string(), Arc::clone(&serial_service));
        let dock_state = DockState::new(vec![initial_tab]);
        Self {
            dock_state,
            counter: 1,
            serial_service,
            theme: Theme::default(),
        }
    }
}

impl eframe::App for MyApp {
    // テーマの反映
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.theme {
            Theme::System => {
                if let Some(theme) = ctx.input(|i| i.raw.system_theme) {
                    match theme {
                        egui::Theme::Light => ctx.set_visuals(egui::Visuals::light()),
                        egui::Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
                    }
                }
            }
            Theme::LightMode => ctx.set_visuals(egui::Visuals::light()),
            Theme::DarkMode => ctx.set_visuals(egui::Visuals::dark()),
        }

        // トップメニュー
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Preferences", |ui| {
                    ui.menu_button("Theme", |ui| {
                        ui.selectable_value(&mut self.theme, Theme::System, "System");
                        ui.selectable_value(&mut self.theme, Theme::LightMode, "Light");
                        ui.selectable_value(&mut self.theme, Theme::DarkMode, "Dark");
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                });
            });
        });

        let mut added_nodes = Vec::new();

        // DockArea の設定
        let mut style = Style::from_egui(ctx.style().as_ref());
        style.buttons.add_tab_align = TabAddAlign::Left;

        DockArea::new(&mut self.dock_state)
            .style(style)
            .show_add_buttons(true)
            .show_leaf_close_all_buttons(false)
            .show_leaf_collapse_buttons(false)
            .show(
                ctx,
                &mut AppTabViewer {
                    add_nodes: &mut added_nodes,
                },
            );

        added_nodes.drain(..).for_each(|(surface, node)| {
            self.dock_state
                .set_focused_node_and_surface((surface, node));
            let new_tab = ui::SerialView::new(
                format!("Port {}", self.counter),
                Arc::clone(&self.serial_service),
            );
            self.dock_state.push_to_focused_leaf(new_tab);
            self.counter += 1;
        });

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
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    );
}
