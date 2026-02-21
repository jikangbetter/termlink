//! 终端渲染器
//! 负责在egui中渲染终端内容

use crate::terminal::theme::TerminalTheme;
use eframe::egui;

/// 终端字符单元
#[derive(Debug, Clone)]
pub struct TerminalCell {
    pub character: char,
    pub fg_color: egui::Color32,
    pub bg_color: egui::Color32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for TerminalCell {
    fn default() -> Self {
        Self {
            character: ' ',
            fg_color: egui::Color32::WHITE,
            bg_color: egui::Color32::BLACK,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

/// 终端缓冲区
#[derive(Debug, Clone)]
pub struct TerminalBuffer {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<TerminalCell>,
    pub cursor_row: usize,
    pub cursor_col: usize,
}

impl TerminalBuffer {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cell_count = rows * cols;
        let cells = vec![TerminalCell::default(); cell_count];

        Self {
            rows,
            cols,
            cells,
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        let new_cell_count = new_rows * new_cols;
        self.cells.resize(new_cell_count, TerminalCell::default());
        self.rows = new_rows;
        self.cols = new_cols;

        // 确保光标在有效范围内
        self.cursor_row = self.cursor_row.min(new_rows - 1);
        self.cursor_col = self.cursor_col.min(new_cols - 1);
    }

    pub fn get_cell_index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut TerminalCell> {
        self.get_cell_index(row, col)
            .and_then(|index| self.cells.get_mut(index))
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&TerminalCell> {
        self.get_cell_index(row, col)
            .and_then(|index| self.cells.get(index))
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        if row < self.rows && col < self.cols {
            self.cursor_row = row;
            self.cursor_col = col;
        }
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = TerminalCell::default();
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
    }
}

/// 终端渲染器
pub struct TerminalRenderer {
    pub buffer: TerminalBuffer,
    pub theme: TerminalTheme,
    pub font_id: egui::FontId,
}

impl TerminalRenderer {
    pub fn new(rows: usize, cols: usize, theme: TerminalTheme) -> Self {
        let buffer = TerminalBuffer::new(rows, cols);
        let font_id = egui::FontId::monospace(theme.font_size);

        Self {
            buffer,
            theme,
            font_id,
        }
    }

    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.buffer.resize(rows, cols);
    }

    pub fn render(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let available_size = ui.available_size();
        let char_size = self.measure_char_size(ui);

        let actual_rows = (available_size.y / (char_size.y * self.theme.line_height)) as usize;
        let actual_cols = (available_size.x / char_size.x) as usize;

        // 调整缓冲区大小
        if actual_rows != self.buffer.rows || actual_cols != self.buffer.cols {
            self.resize(actual_rows.max(1), actual_cols.max(1));
        }

        // 计算实际渲染区域
        let render_width = actual_cols as f32 * char_size.x;
        let render_height = actual_rows as f32 * char_size.y * self.theme.line_height;

        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(render_width, render_height),
            egui::Sense::click_and_drag(),
        );

        if ui.is_rect_visible(rect) {
            let mut painter = ui.painter_at(rect);

            // 绘制背景
            painter.rect_filled(rect, 0.0, self.theme.style.background);

            // 绘制字符
            for row in 0..self.buffer.rows {
                for col in 0..self.buffer.cols {
                    if let Some(cell) = self.buffer.get_cell(row, col) {
                        let char_pos = egui::pos2(
                            rect.min.x + col as f32 * char_size.x,
                            rect.min.y + row as f32 * char_size.y * self.theme.line_height,
                        );

                        // 绘制背景色
                        if cell.bg_color != self.theme.style.background {
                            let bg_rect = egui::Rect::from_min_size(
                                char_pos,
                                egui::vec2(char_size.x, char_size.y * self.theme.line_height),
                            );
                            painter.rect_filled(bg_rect, 0.0, cell.bg_color);
                        }

                        // 绘制字符
                        let text_color = if cell.fg_color == egui::Color32::WHITE
                            && self.theme.style.foreground != egui::Color32::WHITE
                        {
                            self.theme.style.foreground
                        } else {
                            cell.fg_color
                        };

                        painter.text(
                            char_pos,
                            egui::Align2::LEFT_TOP,
                            cell.character.to_string(),
                            self.font_id.clone(),
                            text_color,
                        );
                    }
                }
            }

            // 绘制光标
            self.render_cursor(&mut painter, rect, char_size);
        }

        response
    }

    fn measure_char_size(&self, ui: &mut egui::Ui) -> egui::Vec2 {
        let layout_job = egui::text::LayoutJob::single_section(
            "W".to_string(),
            egui::TextFormat::simple(self.font_id.clone(), egui::Color32::WHITE),
        );
        let galley = ui.fonts(|f| f.layout_job(layout_job));
        egui::vec2(galley.size().x, galley.size().y)
    }

    fn render_cursor(&self, painter: &mut egui::Painter, rect: egui::Rect, char_size: egui::Vec2) {
        if self.buffer.cursor_row < self.buffer.rows && self.buffer.cursor_col < self.buffer.cols {
            let cursor_pos = egui::pos2(
                rect.min.x + self.buffer.cursor_col as f32 * char_size.x,
                rect.min.y + self.buffer.cursor_row as f32 * char_size.y * self.theme.line_height,
            );

            let cursor_rect = egui::Rect::from_min_size(
                cursor_pos,
                egui::vec2(char_size.x, char_size.y * self.theme.line_height),
            );

            // 简单的块状光标
            painter.rect_filled(cursor_rect, 0.0, self.theme.style.cursor);
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
