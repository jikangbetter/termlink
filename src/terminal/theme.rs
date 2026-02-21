//! 终端主题配置
//! 管理终端的颜色主题和样式

use egui::Color32;

/// 终端主题样式
#[derive(Debug, Clone)]
pub struct ThemeStyle {
    pub foreground: Color32,
    pub background: Color32,
    pub cursor: Color32,
    pub selection: Color32,
    pub black: Color32,
    pub red: Color32,
    pub green: Color32,
    pub yellow: Color32,
    pub blue: Color32,
    pub magenta: Color32,
    pub cyan: Color32,
    pub white: Color32,
    pub bright_black: Color32,
    pub bright_red: Color32,
    pub bright_green: Color32,
    pub bright_yellow: Color32,
    pub bright_blue: Color32,
    pub bright_magenta: Color32,
    pub bright_cyan: Color32,
    pub bright_white: Color32,
}

impl Default for ThemeStyle {
    fn default() -> Self {
        Self::dark()
    }
}

impl ThemeStyle {
    /// 深色主题
    pub fn dark() -> Self {
        Self {
            foreground: Color32::WHITE,
            background: Color32::from_rgb(15, 15, 15),
            cursor: Color32::WHITE,
            selection: Color32::from_rgba_premultiplied(255, 255, 255, 40),
            black: Color32::from_rgb(0, 0, 0),
            red: Color32::from_rgb(205, 49, 49),
            green: Color32::from_rgb(13, 188, 121),
            yellow: Color32::from_rgb(229, 229, 16),
            blue: Color32::from_rgb(36, 114, 200),
            magenta: Color32::from_rgb(188, 63, 188),
            cyan: Color32::from_rgb(17, 168, 205),
            white: Color32::from_rgb(229, 229, 229),
            bright_black: Color32::from_rgb(102, 102, 102),
            bright_red: Color32::from_rgb(241, 76, 76),
            bright_green: Color32::from_rgb(35, 209, 139),
            bright_yellow: Color32::from_rgb(245, 245, 67),
            bright_blue: Color32::from_rgb(59, 142, 234),
            bright_magenta: Color32::from_rgb(214, 112, 214),
            bright_cyan: Color32::from_rgb(41, 184, 219),
            bright_white: Color32::from_rgb(229, 229, 229),
        }
    }

    /// 浅色主题
    pub fn light() -> Self {
        Self {
            foreground: Color32::BLACK,
            background: Color32::WHITE,
            cursor: Color32::BLACK,
            selection: Color32::from_rgba_premultiplied(0, 0, 0, 40),
            black: Color32::from_rgb(0, 0, 0),
            red: Color32::from_rgb(160, 32, 32),
            green: Color32::from_rgb(0, 128, 0),
            yellow: Color32::from_rgb(128, 128, 0),
            blue: Color32::from_rgb(0, 0, 160),
            magenta: Color32::from_rgb(128, 0, 128),
            cyan: Color32::from_rgb(0, 128, 128),
            white: Color32::from_rgb(192, 192, 192),
            bright_black: Color32::from_rgb(128, 128, 128),
            bright_red: Color32::from_rgb(255, 0, 0),
            bright_green: Color32::from_rgb(0, 255, 0),
            bright_yellow: Color32::from_rgb(255, 255, 0),
            bright_blue: Color32::from_rgb(0, 0, 255),
            bright_magenta: Color32::from_rgb(255, 0, 255),
            bright_cyan: Color32::from_rgb(0, 255, 255),
            bright_white: Color32::from_rgb(255, 255, 255),
        }
    }
}

/// 终端主题
#[derive(Debug, Clone)]
pub struct TerminalTheme {
    pub style: ThemeStyle,
    pub font_size: f32,
    pub line_height: f32,
    pub cursor_blink: bool,
}

impl Default for TerminalTheme {
    fn default() -> Self {
        Self {
            style: ThemeStyle::default(),
            font_size: 14.0,
            line_height: 1.2,
            cursor_blink: true,
        }
    }
}

impl TerminalTheme {
    pub fn new(style: ThemeStyle, font_size: f32) -> Self {
        Self {
            style,
            font_size,
            line_height: 1.2,
            cursor_blink: true,
        }
    }

    /// 获取指定颜色索引的颜色
    pub fn get_color(&self, index: u8, intense: bool) -> Color32 {
        match (index, intense) {
            (0, false) => self.style.black,
            (0, true) => self.style.bright_black,
            (1, false) => self.style.red,
            (1, true) => self.style.bright_red,
            (2, false) => self.style.green,
            (2, true) => self.style.bright_green,
            (3, false) => self.style.yellow,
            (3, true) => self.style.bright_yellow,
            (4, false) => self.style.blue,
            (4, true) => self.style.bright_blue,
            (5, false) => self.style.magenta,
            (5, true) => self.style.bright_magenta,
            (6, false) => self.style.cyan,
            (6, true) => self.style.bright_cyan,
            (7, false) => self.style.white,
            (7, true) => self.style.bright_white,
            _ => self.style.foreground,
        }
    }
}
