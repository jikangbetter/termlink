//! 终端主题配置
//! 管理终端的颜色主题和样式

use egui::Color32;

/// 256色调色板
const COLOR_256: [(u8, u8, u8); 256] = [
    // 标准颜色 (0-15)
    (0, 0, 0),
    (128, 0, 0),
    (0, 128, 0),
    (128, 128, 0),
    (0, 0, 128),
    (128, 0, 128),
    (0, 128, 128),
    (192, 192, 192),
    (128, 128, 128),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (0, 0, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
    // 6x6x6 立方体颜色 (16-231)
    (0, 0, 0),
    (0, 0, 95),
    (0, 0, 135),
    (0, 0, 175),
    (0, 0, 215),
    (0, 0, 255),
    (0, 95, 0),
    (0, 95, 95),
    (0, 95, 135),
    (0, 95, 175),
    (0, 95, 215),
    (0, 95, 255),
    (0, 135, 0),
    (0, 135, 95),
    (0, 135, 135),
    (0, 135, 175),
    (0, 135, 215),
    (0, 135, 255),
    (0, 175, 0),
    (0, 175, 95),
    (0, 175, 135),
    (0, 175, 175),
    (0, 175, 215),
    (0, 175, 255),
    (0, 215, 0),
    (0, 215, 95),
    (0, 215, 135),
    (0, 215, 175),
    (0, 215, 215),
    (0, 215, 255),
    (0, 255, 0),
    (0, 255, 95),
    (0, 255, 135),
    (0, 255, 175),
    (0, 255, 215),
    (0, 255, 255),
    (95, 0, 0),
    (95, 0, 95),
    (95, 0, 135),
    (95, 0, 175),
    (95, 0, 215),
    (95, 0, 255),
    (95, 95, 0),
    (95, 95, 95),
    (95, 95, 135),
    (95, 95, 175),
    (95, 95, 215),
    (95, 95, 255),
    (95, 135, 0),
    (95, 135, 95),
    (95, 135, 135),
    (95, 135, 175),
    (95, 135, 215),
    (95, 135, 255),
    (95, 175, 0),
    (95, 175, 95),
    (95, 175, 135),
    (95, 175, 175),
    (95, 175, 215),
    (95, 175, 255),
    (95, 215, 0),
    (95, 215, 95),
    (95, 215, 135),
    (95, 215, 175),
    (95, 215, 215),
    (95, 215, 255),
    (95, 255, 0),
    (95, 255, 95),
    (95, 255, 135),
    (95, 255, 175),
    (95, 255, 215),
    (95, 255, 255),
    (135, 0, 0),
    (135, 0, 95),
    (135, 0, 135),
    (135, 0, 175),
    (135, 0, 215),
    (135, 0, 255),
    (135, 95, 0),
    (135, 95, 95),
    (135, 95, 135),
    (135, 95, 175),
    (135, 95, 215),
    (135, 95, 255),
    (135, 135, 0),
    (135, 135, 95),
    (135, 135, 135),
    (135, 135, 175),
    (135, 135, 215),
    (135, 135, 255),
    (135, 175, 0),
    (135, 175, 95),
    (135, 175, 135),
    (135, 175, 175),
    (135, 175, 215),
    (135, 175, 255),
    (135, 215, 0),
    (135, 215, 95),
    (135, 215, 135),
    (135, 215, 175),
    (135, 215, 215),
    (135, 215, 255),
    (135, 255, 0),
    (135, 255, 95),
    (135, 255, 135),
    (135, 255, 175),
    (135, 255, 215),
    (135, 255, 255),
    (175, 0, 0),
    (175, 0, 95),
    (175, 0, 135),
    (175, 0, 175),
    (175, 0, 215),
    (175, 0, 255),
    (175, 95, 0),
    (175, 95, 95),
    (175, 95, 135),
    (175, 95, 175),
    (175, 95, 215),
    (175, 95, 255),
    (175, 135, 0),
    (175, 135, 95),
    (175, 135, 135),
    (175, 135, 175),
    (175, 135, 215),
    (175, 135, 255),
    (175, 175, 0),
    (175, 175, 95),
    (175, 175, 135),
    (175, 175, 175),
    (175, 175, 215),
    (175, 175, 255),
    (175, 215, 0),
    (175, 215, 95),
    (175, 215, 135),
    (175, 215, 175),
    (175, 215, 215),
    (175, 215, 255),
    (175, 255, 0),
    (175, 255, 95),
    (175, 255, 135),
    (175, 255, 175),
    (175, 255, 215),
    (175, 255, 255),
    (215, 0, 0),
    (215, 0, 95),
    (215, 0, 135),
    (215, 0, 175),
    (215, 0, 215),
    (215, 0, 255),
    (215, 95, 0),
    (215, 95, 95),
    (215, 95, 135),
    (215, 95, 175),
    (215, 95, 215),
    (215, 95, 255),
    (215, 135, 0),
    (215, 135, 95),
    (215, 135, 135),
    (215, 135, 175),
    (215, 135, 215),
    (215, 135, 255),
    (215, 175, 0),
    (215, 175, 95),
    (215, 175, 135),
    (215, 175, 175),
    (215, 175, 215),
    (215, 175, 255),
    (215, 215, 0),
    (215, 215, 95),
    (215, 215, 135),
    (215, 215, 175),
    (215, 215, 215),
    (215, 215, 255),
    (215, 255, 0),
    (215, 255, 95),
    (215, 255, 135),
    (215, 255, 175),
    (215, 255, 215),
    (215, 255, 255),
    (255, 0, 0),
    (255, 0, 95),
    (255, 0, 135),
    (255, 0, 175),
    (255, 0, 215),
    (255, 0, 255),
    (255, 95, 0),
    (255, 95, 95),
    (255, 95, 135),
    (255, 95, 175),
    (255, 95, 215),
    (255, 95, 255),
    (255, 135, 0),
    (255, 135, 95),
    (255, 135, 135),
    (255, 135, 175),
    (255, 135, 215),
    (255, 135, 255),
    (255, 175, 0),
    (255, 175, 95),
    (255, 175, 135),
    (255, 175, 175),
    (255, 175, 215),
    (255, 175, 255),
    (255, 215, 0),
    (255, 215, 95),
    (255, 215, 135),
    (255, 215, 175),
    (255, 215, 215),
    (255, 215, 255),
    (255, 255, 0),
    (255, 255, 95),
    (255, 255, 135),
    (255, 255, 175),
    (255, 255, 215),
    (255, 255, 255),
    // 灰度颜色 (232-255)
    (8, 8, 8),
    (18, 18, 18),
    (28, 28, 28),
    (38, 38, 38),
    (48, 48, 48),
    (58, 58, 58),
    (68, 68, 68),
    (78, 78, 78),
    (88, 88, 88),
    (98, 98, 98),
    (108, 108, 108),
    (118, 118, 118),
    (128, 128, 128),
    (138, 138, 138),
    (148, 148, 148),
    (158, 158, 158),
    (168, 168, 168),
    (178, 178, 178),
    (188, 188, 188),
    (198, 198, 198),
    (208, 208, 208),
    (218, 218, 218),
    (228, 228, 228),
    (238, 238, 238),
];

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
    /// 从16进制字符串解析颜色 (#RRGGBB 或 #RRGGBBAA)
    pub fn parse_hex(hex: &str) -> Color32 {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            Color32::from_rgb(r, g, b)
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            Color32::from_rgba_premultiplied(r, g, b, a)
        } else {
            Color32::WHITE
        }
    }

    /// 深色主题
    pub fn dark() -> Self {
        Self {
            foreground: Color32::WHITE,
            background: Color32::from_rgb(15, 15, 15),
            cursor: Color32::WHITE,
            selection: Color32::from_rgba_premultiplied(100, 149, 237, 180), // 深天蓝色，更明显
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
            selection: Color32::from_rgba_premultiplied(173, 216, 230, 180), // 浅蓝色，更明显
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

    /// 获取256色调色板中的颜色
    pub fn get_color_256(&self, index: u16) -> Color32 {
        if index < 256 {
            let (r, g, b) = COLOR_256[index as usize];
            Color32::from_rgb(r, g, b)
        } else {
            self.style.foreground
        }
    }

    /// 解析RGB颜色值
    pub fn parse_rgb_color(&self, r: u8, g: u8, b: u8) -> Color32 {
        Color32::from_rgb(r, g, b)
    }

    /// 解析颜色代码（支持多种格式）
    pub fn parse_color_code(&self, code: u8, params: &[i64]) -> Option<Color32> {
        match code {
            // 标准颜色 (30-37, 40-47)
            30..=37 => Some(self.get_color(code - 30, false)),
            40..=47 => Some(self.get_color(code - 40, false)),
            // 亮色 (90-97, 100-107)
            90..=97 => Some(self.get_color(code - 90 + 8, true)),
            100..=107 => Some(self.get_color(code - 100 + 8, true)),
            // 256色 (38, 48)
            38 | 48 => {
                if params.len() >= 2 {
                    match params[0] {
                        5 => {
                            // 256色
                            if params.len() >= 2 {
                                let color_index = params[1] as u16;
                                Some(self.get_color_256(color_index))
                            } else {
                                None
                            }
                        }
                        2 => {
                            // RGB颜色
                            if params.len() >= 4 {
                                let r = params[1] as u8;
                                let g = params[2] as u8;
                                let b = params[3] as u8;
                                Some(self.parse_rgb_color(r, g, b))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
