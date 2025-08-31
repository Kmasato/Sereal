pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    #[allow(dead_code)]
    pub transparent: u8,
}

impl Color {
    pub fn to_egui_color32(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.red, self.green, self.blue)
    }
}
