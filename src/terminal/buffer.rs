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
    /// 是否为宽字符的延续位
    pub is_continuation: bool,
}

impl Default for TerminalCell {
    fn default() -> Self {
        Self {
            character: ' ',
            fg_color: egui::Color32::WHITE,
            bg_color: egui::Color32::TRANSPARENT,
            bold: false,
            italic: false,
            underline: false,
            is_continuation: false,
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
    /// 历史回溯缓冲区 (最多存储1000行)
    pub history: Vec<Vec<TerminalCell>>,
    pub max_history: usize,
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
            history: Vec::new(),
            max_history: 1000,
        }
    }

    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        if new_rows == self.rows && new_cols == self.cols {
            return;
        }

        let mut new_cells = vec![TerminalCell::default(); new_rows * new_cols];

        // 复制旧内容
        for r in 0..self.rows.min(new_rows) {
            for c in 0..self.cols.min(new_cols) {
                if let Some(old_idx) = self.get_cell_index(r, c) {
                    new_cells[r * new_cols + c] = self.cells[old_idx].clone();
                }
            }
        }

        self.cells = new_cells;
        self.rows = new_rows;
        self.cols = new_cols;

        // 确保光标在有效范围内
        self.cursor_row = self.cursor_row.min(new_rows.saturating_sub(1));
        self.cursor_col = self.cursor_col.min(new_cols.saturating_sub(1));
    }

    pub fn get_cell_index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut TerminalCell> {
        let idx = self.get_cell_index(row, col)?;
        Some(&mut self.cells[idx])
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&TerminalCell> {
        let idx = self.get_cell_index(row, col)?;
        Some(&self.cells[idx])
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

    pub fn newline(&mut self) {
        if self.cursor_row < self.rows - 1 {
            self.cursor_row += 1;
        } else {
            // 将顶行推入历史回溯缓冲区
            let top_line = self.cells[0..self.cols].to_vec();
            self.history.push(top_line);
            if self.history.len() > self.max_history {
                self.history.remove(0);
            }

            // 屏幕整体向上滚动
            self.cells.drain(0..self.cols);
            self.cells.extend(vec![TerminalCell::default(); self.cols]);
        }
    }

    pub fn carriage_return(&mut self) {
        self.cursor_col = 0;
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }
}
