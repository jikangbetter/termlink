//! 终端渲染器
//! 负责在egui中渲染终端内容

use crate::terminal::buffer::TerminalBuffer;
use crate::terminal::theme::TerminalTheme;
use eframe::egui;

/// 终端渲染器
pub struct TerminalRenderer {
    pub buffer: TerminalBuffer,
    pub theme: TerminalTheme,
    pub font_id: egui::FontId,
    /// 光标闪烁计时器
    cursor_blink_timer: std::time::Instant,
    /// 光标是否可见
    cursor_visible: bool,
}

impl TerminalRenderer {
    pub fn new(rows: usize, cols: usize, theme: TerminalTheme) -> Self {
        let buffer = TerminalBuffer::new(rows, cols);
        let font_id = egui::FontId::monospace(theme.font_size);

        Self {
            buffer,
            theme,
            font_id,
            cursor_blink_timer: std::time::Instant::now(),
            cursor_visible: true,
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.buffer.resize(rows, cols);
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let char_size = self.measure_char_size(ui);

        // 计算总高度（包括历史记录和当前可见行）
        let total_rows = self.buffer.history.len() + self.buffer.rows;
        let total_height = total_rows as f32 * char_size.y * self.theme.line_height;
        let render_width = self.buffer.cols as f32 * char_size.x;

        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(render_width, total_height),
            egui::Sense::click_and_drag(),
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);

            // 绘制整个背景
            painter.rect_filled(rect, 0.0, self.theme.style.background);

            // 1. 绘制历史记录
            for (h_row, line) in self.buffer.history.iter().enumerate() {
                let row_pos_y = rect.min.y + h_row as f32 * char_size.y * self.theme.line_height;

                // 检查行是否可见
                let row_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.min.x, row_pos_y),
                    egui::vec2(render_width, char_size.y * self.theme.line_height),
                );

                if !ui.is_rect_visible(row_rect) {
                    continue;
                }

                for col in 0..self.buffer.cols {
                    if let Some(cell) = line.get(col) {
                        if cell.is_continuation {
                            continue;
                        }

                        if cell.character == ' ' && cell.bg_color == egui::Color32::TRANSPARENT {
                            continue;
                        }

                        let width = crate::utils::helpers::get_char_width(cell.character);
                        let char_pos = egui::pos2(rect.min.x + col as f32 * char_size.x, row_pos_y);

                        // 绘制背景（包括选中状态）
                        let bg_color = if cell.is_selected {
                            // 选中状态的背景色
                            self.theme.style.selection
                        } else if cell.bg_color != egui::Color32::TRANSPARENT
                            && cell.bg_color != self.theme.style.background
                        {
                            cell.bg_color
                        } else {
                            self.theme.style.background
                        };

                        if bg_color != self.theme.style.background {
                            let bg_rect = egui::Rect::from_min_size(
                                char_pos,
                                egui::vec2(
                                    char_size.x * width as f32,
                                    char_size.y * self.theme.line_height,
                                ),
                            );
                            painter.rect_filled(bg_rect, 0.0, bg_color);
                        }

                        // 绘制文字
                        if cell.character != ' ' {
                            painter.text(
                                char_pos,
                                egui::Align2::LEFT_TOP,
                                cell.character.to_string(),
                                self.font_id.clone(),
                                cell.fg_color,
                            );
                        }
                    }
                }
            }

            // 2. 绘制当前可视屏幕
            let screen_start_y = rect.min.y
                + self.buffer.history.len() as f32 * char_size.y * self.theme.line_height;
            for row in 0..self.buffer.rows {
                let row_pos_y = screen_start_y + row as f32 * char_size.y * self.theme.line_height;

                let row_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.min.x, row_pos_y),
                    egui::vec2(render_width, char_size.y * self.theme.line_height),
                );

                if !ui.is_rect_visible(row_rect) {
                    continue;
                }

                for col in 0..self.buffer.cols {
                    if let Some(cell) = self.buffer.get_cell(row, col) {
                        if cell.is_continuation {
                            continue;
                        }

                        if cell.character == ' ' && cell.bg_color == egui::Color32::TRANSPARENT {
                            continue;
                        }

                        let width = crate::utils::helpers::get_char_width(cell.character);
                        let char_pos = egui::pos2(rect.min.x + col as f32 * char_size.x, row_pos_y);

                        // 绘制背景（包括选中状态）
                        let bg_color = if cell.is_selected {
                            // 选中状态的背景色
                            self.theme.style.selection
                        } else if cell.bg_color != egui::Color32::TRANSPARENT
                            && cell.bg_color != self.theme.style.background
                        {
                            cell.bg_color
                        } else {
                            self.theme.style.background
                        };

                        if bg_color != self.theme.style.background {
                            let bg_rect = egui::Rect::from_min_size(
                                char_pos,
                                egui::vec2(
                                    char_size.x * width as f32,
                                    char_size.y * self.theme.line_height,
                                ),
                            );
                            painter.rect_filled(bg_rect, 0.0, bg_color);
                        }

                        if cell.character != ' ' {
                            painter.text(
                                char_pos,
                                egui::Align2::LEFT_TOP,
                                cell.character.to_string(),
                                self.font_id.clone(),
                                cell.fg_color,
                            );
                        }
                    }
                }
            }

            // 绘制光标 (在其相对于当前屏幕的位置绘制)
            let cursor_screen_pos = egui::pos2(rect.min.x, screen_start_y);
            self.render_cursor(&painter, cursor_screen_pos, char_size);
        }

        response
    }

    fn measure_char_size(&self, ui: &mut egui::Ui) -> egui::Vec2 {
        // 使用更精确的测量方式，通过绘制一个测试字符来获取尺寸
        let font_id = self.font_id.clone();
        let galley = ui
            .painter()
            .layout_no_wrap("W".to_string(), font_id, egui::Color32::WHITE);

        let char_width = galley.size().x;
        let row_height = galley.size().y;

        egui::vec2(char_width, row_height)
    }

    fn render_cursor(
        &mut self,
        painter: &egui::Painter,
        screen_origin: egui::Pos2,
        char_size: egui::Vec2,
    ) {
        if self.buffer.cursor_row < self.buffer.rows && self.buffer.cursor_col < self.buffer.cols {
            // 处理光标闪烁
            if self.theme.cursor_blink {
                let elapsed = self.cursor_blink_timer.elapsed().as_millis();
                // 500ms切换一次可见性
                if elapsed > 500 {
                    self.cursor_visible = !self.cursor_visible;
                    self.cursor_blink_timer = std::time::Instant::now();
                }

                // 如果光标不可见，直接返回
                if !self.cursor_visible {
                    return;
                }
            }

            // 获取当前光标位置字符的宽度
            let width = if let Some(cell) = self
                .buffer
                .get_cell(self.buffer.cursor_row, self.buffer.cursor_col)
            {
                if cell.is_continuation {
                    1
                } else {
                    crate::utils::helpers::get_char_width(cell.character)
                }
            } else {
                1
            };

            let cursor_pos = egui::pos2(
                screen_origin.x + self.buffer.cursor_col as f32 * char_size.x,
                screen_origin.y
                    + self.buffer.cursor_row as f32 * char_size.y * self.theme.line_height,
            );

            let cursor_rect = egui::Rect::from_min_size(
                cursor_pos,
                egui::vec2(
                    char_size.x * width as f32,
                    char_size.y * self.theme.line_height,
                ),
            );

            painter.rect_filled(
                cursor_rect,
                0.0,
                self.theme.style.cursor.linear_multiply(0.7),
            );
        }
    }

    /// 更新缓冲区内容
    pub fn update_buffer(&mut self, content: &str) {
        let mut row = 0;
        let mut col = 0;

        for ch in content.chars() {
            match ch {
                '\n' => {
                    row += 1;
                    col = 0;
                    if row >= self.buffer.rows {
                        // 滚动逻辑可以在这里实现
                        row = self.buffer.rows - 1;
                    }
                }
                '\r' => {
                    col = 0;
                }
                '\t' => {
                    col = (col / 8 + 1) * 8;
                    if col >= self.buffer.cols {
                        col = self.buffer.cols - 1;
                    }
                }
                ch => {
                    if row < self.buffer.rows && col < self.buffer.cols {
                        if let Some(cell) = self.buffer.get_cell_mut(row, col) {
                            cell.character = ch;
                            cell.fg_color = self.theme.style.foreground;
                            cell.bg_color = self.theme.style.background;
                        }
                        col += 1;
                        if col >= self.buffer.cols {
                            col = 0;
                            row += 1;
                            if row >= self.buffer.rows {
                                row = self.buffer.rows - 1;
                            }
                        }
                    }
                }
            }
        }

        self.buffer.set_cursor(row, col);
    }
}
