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
    /// 是否被选中（用于显示）
    pub is_selected: bool,
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
            is_selected: false,
        }
    }
}

/// 选择范围
#[derive(Debug, Clone, Default)]
pub struct SelectionRange {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
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
    /// 当前选择范围
    pub selection: Option<SelectionRange>,
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
            selection: None,
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
        // 重置光标列到行首
        self.cursor_col = 0;
    }

    pub fn carriage_return(&mut self) {
        self.cursor_col = 0;
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        }
    }

    /// 开始选择
    pub fn start_selection(&mut self, row: usize, col: usize) {
        self.selection = Some(SelectionRange {
            start_row: row,
            start_col: col,
            end_row: row,
            end_col: col,
        });
        self.update_selection_display();
    }

    /// 更新选择结束位置
    pub fn update_selection(&mut self, row: usize, col: usize) {
        if let Some(ref mut selection) = self.selection {
            selection.end_row = row;
            selection.end_col = col;
            self.update_selection_display();
        }
    }

    /// 结束选择
    pub fn end_selection(&mut self) {
        // 选择已经通过update_selection更新，这里可以添加额外逻辑
    }

    /// 取消选择
    pub fn clear_selection(&mut self) {
        self.selection = None;
        self.clear_selection_display();
    }

    /// 更新选中状态显示
    fn update_selection_display(&mut self) {
        // 先清除旧的选择显示
        self.clear_selection_display();

        if let Some(ref selection) = self.selection {
            let (start_row, start_col, end_row, end_col) = self.normalize_selection(selection);
            let history_len = self.history.len();

            for r in start_row..=end_row {
                let col_start = if r == start_row { start_col } else { 0 };
                let col_end = if r == end_row {
                    end_col
                } else {
                    self.cols.saturating_sub(1)
                };

                if r < history_len {
                    // 处理历史记录
                    if let Some(line) = self.history.get_mut(r) {
                        for c in col_start..=col_end.min(line.len().saturating_sub(1)) {
                            line[c].is_selected = true;
                        }
                    }
                } else {
                    // 处理当前屏幕
                    let screen_r = r - history_len;
                    if screen_r < self.rows {
                        for c in col_start..=col_end.min(self.cols.saturating_sub(1)) {
                            if let Some(cell) = self.get_cell_mut(screen_r, c) {
                                cell.is_selected = true;
                            }
                        }
                    }
                }
            }
        }
    }

    /// 清除选中状态显示
    fn clear_selection_display(&mut self) {
        for cell in &mut self.cells {
            cell.is_selected = false;
        }
        // 清除历史记录中的选中状态
        for line in &mut self.history {
            for cell in line {
                cell.is_selected = false;
            }
        }
    }

    /// 标准化选择范围（确保start <= end）
    fn normalize_selection(&self, selection: &SelectionRange) -> (usize, usize, usize, usize) {
        let (start_row, end_row) = if selection.start_row <= selection.end_row {
            (selection.start_row, selection.end_row)
        } else {
            (selection.end_row, selection.start_row)
        };

        let (start_col, end_col) = if selection.start_row == selection.end_row {
            // 同一行，比较列
            if selection.start_col <= selection.end_col {
                (selection.start_col, selection.end_col)
            } else {
                (selection.end_col, selection.start_col)
            }
        } else {
            // 不同行，按行排序
            if selection.start_row <= selection.end_row {
                (selection.start_col, selection.end_col)
            } else {
                (selection.end_col, selection.start_col)
            }
        };

        (start_row, start_col, end_row, end_col)
    }

    /// 获取选中的文本
    pub fn get_selected_text(&self) -> Option<String> {
        if let Some(ref selection) = self.selection {
            let (start_row, start_col, end_row, end_col) = self.normalize_selection(selection);
            let mut result = String::new();
            let history_len = self.history.len();

            for r in start_row..=end_row {
                let col_start = if r == start_row { start_col } else { 0 };
                let col_end = if r == end_row {
                    end_col
                } else {
                    self.cols.saturating_sub(1)
                };

                if r < history_len {
                    // 从历史记录中获取文本
                    if let Some(line) = self.history.get(r) {
                        for c in col_start..=col_end.min(line.len().saturating_sub(1)) {
                            result.push(line[c].character);
                        }
                    }
                } else {
                    // 从当前屏幕获取文本
                    let screen_r = r - history_len;
                    if screen_r < self.rows {
                        for c in col_start..=col_end.min(self.cols.saturating_sub(1)) {
                            if let Some(cell) = self.get_cell(screen_r, c) {
                                result.push(cell.character);
                            }
                        }
                    }
                }

                // 在行尾添加换行符，除非是最后一行
                if r < end_row {
                    result.push('\n');
                }
            }

            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        } else {
            None
        }
    }
}
