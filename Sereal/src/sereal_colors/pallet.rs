use super::color::Color;

// --- ANSI Normal Colors ---
pub const BLACK: Color = Color {
    red: 0,
    green: 0,
    blue: 0,
    transparent: 255,
};
pub const RED: Color = Color {
    red: 128,
    green: 0,
    blue: 0,
    transparent: 255,
};
pub const GREEN: Color = Color {
    red: 0,
    green: 128,
    blue: 0,
    transparent: 255,
};
pub const YELLOW: Color = Color {
    red: 128,
    green: 128,
    blue: 0,
    transparent: 255,
};
pub const BLUE: Color = Color {
    red: 0,
    green: 0,
    blue: 128,
    transparent: 255,
};
pub const MAGENTA: Color = Color {
    red: 128,
    green: 0,
    blue: 128,
    transparent: 255,
};
pub const CYAN: Color = Color {
    red: 0,
    green: 128,
    blue: 128,
    transparent: 255,
};
pub const WHITE: Color = Color {
    red: 192,
    green: 192,
    blue: 192,
    transparent: 255,
};

// --- ANSI Bright Colors ---
pub const BRIGHT_BLACK: Color = Color {
    red: 128,
    green: 128,
    blue: 128,
    transparent: 255,
};
pub const BRIGHT_RED: Color = Color {
    red: 255,
    green: 0,
    blue: 0,
    transparent: 255,
};
pub const BRIGHT_GREEN: Color = Color {
    red: 0,
    green: 255,
    blue: 0,
    transparent: 255,
};
pub const BRIGHT_YELLOW: Color = Color {
    red: 255,
    green: 255,
    blue: 0,
    transparent: 255,
};
pub const BRIGHT_BLUE: Color = Color {
    red: 0,
    green: 0,
    blue: 255,
    transparent: 255,
};
pub const BRIGHT_MAGENTA: Color = Color {
    red: 255,
    green: 0,
    blue: 255,
    transparent: 255,
};
pub const BRIGHT_CYAN: Color = Color {
    red: 0,
    green: 255,
    blue: 255,
    transparent: 255,
};
pub const BRIGHT_WHITE: Color = Color {
    red: 255,
    green: 255,
    blue: 255,
    transparent: 255,
};

// --- UI Colors ---

pub const UI_GREEN: Color = Color {
    red: 0,
    green: 150,
    blue: 0,
    transparent: 255,
};
pub const UI_RED: Color = Color {
    red: 240,
    green: 40,
    blue: 40,
    transparent: 255,
};
pub const UI_WHITE: Color = Color {
    red: 200,
    green: 200,
    blue: 200,
    transparent: 255,
};
