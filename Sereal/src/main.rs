mod serial;
mod ui;

use eframe::egui;
use egui_dock::{DockArea, DockState, Style, TabAddAlign, TabViewer, tab_viewer::OnCloseResponse};

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

pub struct MyApp {
    dock_state: DockState<ui::SerialView>,
    counter: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        let initial_tab = ui::SerialView::default();
        let dock_state = DockState::new(vec![initial_tab]);
        Self {
            dock_state,
            counter: 1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // // トップメニュー
        // egui::TopBottomPanel::top("menu_bar").show(ctx, |_ui| {});

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
            let new_tab = ui::SerialView::new(self.counter);
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
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    );
}
