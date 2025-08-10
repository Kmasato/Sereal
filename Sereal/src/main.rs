mod serial;
mod ui;
use eframe::egui;

pub struct MyApp {
    serial_view: ui::SerialView,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            serial_view: ui::SerialView::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.serial_view.show(ctx);
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
