use crate::sereal_colors;
use ansi_parser::AnsiSequence;
use ansi_parser::{AnsiParser, Output};
use eframe::egui::RichText;

#[derive(Default)]
pub struct AnsiFormatter {
    color_set: ColorSet,
}

impl AnsiFormatter {
    pub fn to_rich_text(&mut self, text: &String) -> Vec<egui::RichText> {
        let mut rich_texts = Vec::new();

        for block in text.ansi_parse() {
            // ANSI タグの結果を反映した RichText
            let mut rich_text = RichText::default();
            // ANSI タグをパースした色情報を管理する構造体
            let mut updated_color_set = ColorSet::default();

            match block {
                Output::TextBlock(text) => rich_text = RichText::from(text),
                Output::Escape(seq) => match seq {
                    AnsiSequence::SetGraphicsMode(params) => {
                        updated_color_set = parse_to_colorset(params.to_vec());
                    }
                    _ => {}
                },
            }

            if let Some(text_color) = updated_color_set.text_color {
                rich_text = rich_text.color(text_color);
                self.color_set.text_color = Some(text_color);
            } else if let Some(text_color) = self.color_set.text_color {
                rich_text = rich_text.color(text_color);
            } else {
                rich_text = rich_text.color(sereal_colors::WHITE.to_egui_color32());
            }

            if let Some(back_color) = updated_color_set.background_color {
                rich_text = rich_text.background_color(back_color);
                self.color_set.background_color = Some(back_color);
            } else if let Some(back_color) = self.color_set.background_color {
                rich_text = rich_text.background_color(back_color);
            }

            if updated_color_set.is_reset {
                self.color_set = ColorSet::default();
            }

            if rich_text.text().len() != 0 {
                rich_texts.push(rich_text);
            }
        }

        rich_texts
    }

    pub fn reset(&mut self) {
        self.color_set = ColorSet::default();
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
struct ColorSet {
    text_color: Option<egui::Color32>,
    background_color: Option<egui::Color32>,
    is_reset: bool,
}

fn parse_to_colorset(graphics_param: Vec<u8>) -> ColorSet {
    let mut color_set = ColorSet::default();

    for param in graphics_param {
        match param {
            // 0: リセット
            0 => {
                return ColorSet {
                    text_color: None,
                    background_color: None,
                    is_reset: true,
                };
            }

            // 30-37: 文字色 (Normal)
            30 => color_set.text_color = Some(sereal_colors::BLACK.to_egui_color32()),
            31 => color_set.text_color = Some(sereal_colors::RED.to_egui_color32()),
            32 => color_set.text_color = Some(sereal_colors::GREEN.to_egui_color32()),
            33 => color_set.text_color = Some(sereal_colors::YELLOW.to_egui_color32()),
            34 => color_set.text_color = Some(sereal_colors::BLUE.to_egui_color32()),
            35 => color_set.text_color = Some(sereal_colors::MAGENTA.to_egui_color32()),
            36 => color_set.text_color = Some(sereal_colors::CYAN.to_egui_color32()),
            37 => color_set.text_color = Some(sereal_colors::WHITE.to_egui_color32()),

            // 40-47: 背景色 (Normal)
            40 => color_set.background_color = Some(sereal_colors::BLACK.to_egui_color32()),
            41 => color_set.background_color = Some(sereal_colors::RED.to_egui_color32()),
            42 => color_set.background_color = Some(sereal_colors::GREEN.to_egui_color32()),
            43 => color_set.background_color = Some(sereal_colors::YELLOW.to_egui_color32()),
            44 => color_set.background_color = Some(sereal_colors::BLUE.to_egui_color32()),
            45 => color_set.background_color = Some(sereal_colors::MAGENTA.to_egui_color32()),
            46 => color_set.background_color = Some(sereal_colors::CYAN.to_egui_color32()),
            47 => color_set.background_color = Some(sereal_colors::WHITE.to_egui_color32()),

            // 90-97: 文字色 (Bright)
            90 => color_set.text_color = Some(sereal_colors::BRIGHT_BLACK.to_egui_color32()),
            91 => color_set.text_color = Some(sereal_colors::BRIGHT_RED.to_egui_color32()),
            92 => color_set.text_color = Some(sereal_colors::BRIGHT_GREEN.to_egui_color32()),
            93 => color_set.text_color = Some(sereal_colors::BRIGHT_YELLOW.to_egui_color32()),
            94 => color_set.text_color = Some(sereal_colors::BRIGHT_BLUE.to_egui_color32()),
            95 => color_set.text_color = Some(sereal_colors::BRIGHT_MAGENTA.to_egui_color32()),
            96 => color_set.text_color = Some(sereal_colors::BRIGHT_CYAN.to_egui_color32()),
            97 => color_set.text_color = Some(sereal_colors::BRIGHT_WHITE.to_egui_color32()),

            // 100-107: 背景色 (Bright)
            100 => color_set.background_color = Some(sereal_colors::BRIGHT_BLACK.to_egui_color32()),
            101 => color_set.background_color = Some(sereal_colors::BRIGHT_RED.to_egui_color32()),
            102 => color_set.background_color = Some(sereal_colors::BRIGHT_GREEN.to_egui_color32()),
            103 => {
                color_set.background_color = Some(sereal_colors::BRIGHT_YELLOW.to_egui_color32())
            }
            104 => color_set.background_color = Some(sereal_colors::BRIGHT_BLUE.to_egui_color32()),
            105 => {
                color_set.background_color = Some(sereal_colors::BRIGHT_MAGENTA.to_egui_color32())
            }
            106 => color_set.background_color = Some(sereal_colors::BRIGHT_CYAN.to_egui_color32()),
            107 => color_set.background_color = Some(sereal_colors::BRIGHT_WHITE.to_egui_color32()),

            // それ以外は何もしない
            _ => {}
        }
    }

    color_set
}
