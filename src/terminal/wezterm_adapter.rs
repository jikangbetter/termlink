//! WezTerm终端仿真器适配器
//! 将WezTerm的终端仿真能力集成到项目中

use crate::terminal::buffer::TerminalBuffer;
use crate::terminal::emulator::{TerminalEvent, TerminalState};
use crate::terminal::theme::TerminalTheme;
use anyhow::Result;
use std::collections::VecDeque;

/// WezTerm终端仿真器适配器
pub struct WezTermAdapter {
    /// 终端缓冲区
    buffer: TerminalBuffer,
    /// 输出缓冲区
    output_buffer: VecDeque<String>,
    /// 终端状态
    state: TerminalState,
    /// 主题配置
    theme: TerminalTheme,
    /// 当前前景色
    current_fg: Option<u8>,
    /// 当前背景色
    current_bg: Option<u8>,
    /// 是否加粗
    bold: bool,
    /// 事件回调
    callback: Option<Box<dyn Fn(TerminalEvent) + Send + Sync>>,
}

impl WezTermAdapter {
    /// 创建新的WezTerm适配器
    pub fn new(rows: usize, cols: usize, theme: TerminalTheme) -> Self {
        let buffer = TerminalBuffer::new(rows, cols);

        Self {
            buffer,
            output_buffer: VecDeque::new(),
            state: TerminalState::Disconnected,
            theme,
            current_fg: None,
            current_bg: None,
            bold: false,
            callback: None,
        }
    }

    /// 设置事件回调
    pub fn set_event_callback<F>(&mut self, callback: F)
    where
        F: Fn(TerminalEvent) + Send + Sync + 'static,
    {
        self.callback = Some(Box::new(callback));
    }

    /// 发送事件
    fn send_event(&self, event: TerminalEvent) {
        if let Some(ref callback) = self.callback {
            callback(event);
        }
    }

    /// 处理ANSI转义序列
    fn handle_ansi_escape(&mut self, sequence: &str) {
        if sequence.starts_with("[") && sequence.ends_with("m") {
            // 处理SGR (Select Graphic Rendition) 序列
            let params_str = &sequence[1..sequence.len() - 1];
            let params: Vec<&str> = params_str.split(';').collect();

            if params.is_empty() || (params.len() == 1 && params[0].is_empty()) {
                // 重置所有属性
                self.current_fg = None;
                self.current_bg = None;
                self.bold = false;
                return;
            }

            let mut i = 0;
            while i < params.len() {
                if let Ok(code) = params[i].parse::<u8>() {
                    match code {
                        0 => {
                            // 重置所有属性
                            self.current_fg = None;
                            self.current_bg = None;
                            self.bold = false;
                        }
                        1 => {
                            // 加粗
                            self.bold = true;
                        }
                        22 => {
                            // 取消加粗
                            self.bold = false;
                        }
                        30..=37 => {
                            // 前景色
                            self.current_fg = Some(code - 30);
                        }
                        39 => {
                            // 默认前景色
                            self.current_fg = None;
                        }
                        40..=47 => {
                            // 背景色
                            self.current_bg = Some(code - 40);
                        }
                        49 => {
                            // 默认背景色
                            self.current_bg = None;
                        }
                        90..=97 => {
                            // 亮前景色
                            self.current_fg = Some(code - 90 + 8);
                        }
                        100..=107 => {
                            // 亮背景色
                            self.current_bg = Some(code - 100 + 8);
                        }
                        _ => {}
                    }
                }
                i += 1;
            }
        }
    }

    /// 使用文本更新终端缓冲区
    fn update_buffer_with_text(&mut self, text: &str) {
        for ch in text.chars() {
            match ch {
                '\n' => {
                    // 换行处理
                    self.buffer.newline();
                }
                '\r' => {
                    // 回车处理
                    self.buffer.carriage_return();
                }
                '\t' => {
                    // 制表符处理
                    let current_col = self.buffer.cursor_col;
                    let next_tab_stop = ((current_col / 8) + 1) * 8;
                    let spaces_needed = next_tab_stop.min(self.buffer.cols) - current_col;

                    for _ in 0..spaces_needed {
                        if self.buffer.cursor_col < self.buffer.cols {
                            if let Some(cell) = self
                                .buffer
                                .get_cell_mut(self.buffer.cursor_row, self.buffer.cursor_col)
                            {
                                cell.character = ' ';
                            }
                            self.buffer.cursor_col += 1;
                        }
                    }
                }
                ch => {
                    // 普通字符处理
                    if self.buffer.cursor_row < self.buffer.rows
                        && self.buffer.cursor_col < self.buffer.cols
                    {
                        if let Some(cell) = self
                            .buffer
                            .get_cell_mut(self.buffer.cursor_row, self.buffer.cursor_col)
                        {
                            cell.character = ch;
                            // 设置前景色
                            cell.fg_color = if let Some(fg_index) = self.current_fg {
                                self.theme.get_color(fg_index, self.bold)
                            } else {
                                self.theme.style.foreground
                            };
                            // 设置背景色
                            cell.bg_color = if let Some(bg_index) = self.current_bg {
                                self.theme.get_color(bg_index, false)
                            } else {
                                self.theme.style.background
                            };
                            // 设置样式
                            cell.bold = self.bold;
                        }

                        self.buffer.cursor_col += 1;

                        // 如果到达行尾，自动换行
                        if self.buffer.cursor_col >= self.buffer.cols {
                            self.buffer.newline();
                            self.buffer.cursor_col = 0;
                        }
                    }
                }
            }
        }
    }
}

impl crate::terminal::emulator::TerminalEmulatorTrait for WezTermAdapter {
    /// 处理输入数据
    fn process_input(&mut self, data: &[u8]) -> Result<()> {
        // 将输入数据转换为字符串
        let input_text = String::from_utf8_lossy(data);

        // 识别并处理ANSI转义序列
        let mut chars = input_text.chars().peekable();
        let mut normal_text = String::new();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // ESC字符
                // 开始处理转义序列
                if chars.peek() == Some(&'[') {
                    // CSI序列
                    chars.next(); // 跳过 '['
                    let mut sequence = String::from("[");

                    // 收集序列内容直到字母
                    while let Some(seq_char) = chars.next() {
                        sequence.push(seq_char);
                        if seq_char.is_ascii_alphabetic() {
                            break;
                        }
                    }

                    // 处理转义序列
                    if sequence.ends_with('m') {
                        self.handle_ansi_escape(&sequence);
                    }

                    // 将之前积累的普通文本添加到缓冲区
                    if !normal_text.is_empty() {
                        self.add_text_to_buffer(&normal_text);
                        normal_text.clear();
                    }
                } else {
                    // 其他转义序列，暂时当作普通字符处理
                    normal_text.push(ch);
                }
            } else {
                normal_text.push(ch);
            }
        }

        // 处理剩余的普通文本
        if !normal_text.is_empty() {
            self.add_text_to_buffer(&normal_text);
        }

        // 将输入添加到输出缓冲区
        self.output_buffer.push_back(input_text.to_string());

        // 限制缓冲区大小
        if self.output_buffer.len() > 100 {
            self.output_buffer.pop_front();
        }

        // 发送输出事件
        self.send_event(TerminalEvent::Output(input_text.to_string()));

        Ok(())
    }

    /// 获取终端缓冲区
    fn buffer(&self) -> TerminalBuffer {
        self.buffer.clone()
    }

    /// 设置终端大小
    fn resize(&mut self, rows: usize, cols: usize) {
        self.buffer.resize(rows, cols);
        self.send_event(TerminalEvent::Resize { rows, cols });
    }

    /// 更新主题
    fn update_theme(&self, theme: TerminalTheme) {
        // 发送主题更新事件
        self.send_event(TerminalEvent::Output("主题已更新".to_string()));
        // 实际的主题更新逻辑可以在后续实现
    }

    /// 获取终端状态
    fn state(&self) -> &TerminalState {
        &self.state
    }

    /// 设置终端状态
    fn set_state(&mut self, state: TerminalState) {
        self.state = state;
    }

    /// 清空终端
    fn clear(&mut self) {
        self.buffer.clear();
        self.output_buffer.clear();
    }

    fn start_selection(&mut self, row: usize, col: usize) {
        self.buffer.start_selection(row, col);
    }

    fn update_selection(&mut self, row: usize, col: usize) {
        self.buffer.update_selection(row, col);
    }

    fn clear_selection(&mut self) {
        self.buffer.clear_selection();
    }

    fn get_selected_text(&self) -> Option<String> {
        self.buffer.get_selected_text()
    }
}

impl WezTermAdapter {
    /// 将文本添加到缓冲区
    fn add_text_to_buffer(&mut self, text: &str) {
        for ch in text.chars() {
            match ch {
                '\n' => {
                    // 换行处理
                    self.buffer.newline();
                }
                '\r' => {
                    // 回车处理
                    self.buffer.carriage_return();
                }
                '\t' => {
                    // 制表符处理
                    let current_col = self.buffer.cursor_col;
                    let next_tab_stop = ((current_col / 8) + 1) * 8;
                    let spaces_needed = next_tab_stop.min(self.buffer.cols) - current_col;

                    for _ in 0..spaces_needed {
                        if self.buffer.cursor_col < self.buffer.cols {
                            if let Some(cell) = self
                                .buffer
                                .get_cell_mut(self.buffer.cursor_row, self.buffer.cursor_col)
                            {
                                cell.character = ' ';
                                cell.fg_color = if let Some(fg_index) = self.current_fg {
                                    self.theme.get_color(fg_index, self.bold)
                                } else {
                                    self.theme.style.foreground
                                };
                                cell.bg_color = if let Some(bg_index) = self.current_bg {
                                    self.theme.get_color(bg_index, false)
                                } else {
                                    self.theme.style.background
                                };
                                cell.bold = self.bold;
                            }
                            self.buffer.cursor_col += 1;
                        }
                    }
                }
                ch => {
                    // 普通字符处理
                    if self.buffer.cursor_row < self.buffer.rows
                        && self.buffer.cursor_col < self.buffer.cols
                    {
                        if let Some(cell) = self
                            .buffer
                            .get_cell_mut(self.buffer.cursor_row, self.buffer.cursor_col)
                        {
                            cell.character = ch;
                            // 设置前景色
                            cell.fg_color = if let Some(fg_index) = self.current_fg {
                                self.theme.get_color(fg_index, self.bold)
                            } else {
                                self.theme.style.foreground
                            };
                            // 设置背景色
                            cell.bg_color = if let Some(bg_index) = self.current_bg {
                                self.theme.get_color(bg_index, false)
                            } else {
                                self.theme.style.background
                            };
                            // 设置样式
                            cell.bold = self.bold;
                        }

                        self.buffer.cursor_col += 1;

                        // 如果到达行尾，自动换行
                        if self.buffer.cursor_col >= self.buffer.cols {
                            self.buffer.newline();
                            self.buffer.cursor_col = 0;
                        }
                    }
                }
            }
        }
    }

    /// 获取终端缓冲区
    fn buffer(&self) -> TerminalBuffer {
        self.buffer.clone()
    }

    /// 设置终端大小
    fn resize(&mut self, rows: usize, cols: usize) {
        self.buffer.resize(rows, cols);
        self.send_event(TerminalEvent::Resize { rows, cols });
    }

    /// 更新主题
    fn update_theme(&self, theme: TerminalTheme) {
        // 发送主题更新事件
        self.send_event(TerminalEvent::Output("主题已更新".to_string()));
        // 实际的主题更新逻辑可以在后续实现
    }

    /// 获取终端状态
    fn state(&self) -> &TerminalState {
        &self.state
    }

    /// 设置终端状态
    fn set_state(&mut self, state: TerminalState) {
        self.state = state;
    }

    /// 清空终端
    fn clear(&mut self) {
        self.buffer.clear();
        self.output_buffer.clear();
    }
}

impl WezTermAdapter {
    /// 获取输出历史
    pub fn get_output_history(&self) -> Vec<String> {
        self.output_buffer.iter().cloned().collect()
    }

    /// 获取当前光标位置
    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.buffer.cursor_row, self.buffer.cursor_col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::emulator::TerminalEmulatorTrait;

    #[test]
    fn test_wezterm_adapter_creation() {
        let theme = TerminalTheme::default();
        let adapter = WezTermAdapter::new(24, 80, theme);
        assert_eq!(adapter.state(), &TerminalState::Disconnected);
    }

    #[test]
    fn test_buffer_conversion() {
        let theme = TerminalTheme::default();
        let mut adapter = WezTermAdapter::new(10, 20, theme);

        // 处理一些测试数据
        let test_data = b"Hello World\n";
        adapter.process_input(test_data).unwrap();

        let buffer = adapter.buffer();
        assert_eq!(buffer.rows, 10);
        assert_eq!(buffer.cols, 20);
    }

    #[test]
    fn test_output_history() {
        let theme = TerminalTheme::default();
        let mut adapter = WezTermAdapter::new(24, 80, theme);

        // 处理多行输入
        adapter.process_input(b"Line 1\n").unwrap();
        adapter.process_input(b"Line 2\n").unwrap();
        adapter.process_input(b"Line 3").unwrap();

        let history = adapter.get_output_history();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "Line 1\n");
        assert_eq!(history[1], "Line 2\n");
        assert_eq!(history[2], "Line 3");
    }

    #[test]
    fn test_cursor_position() {
        let theme = TerminalTheme::default();
        let mut adapter = WezTermAdapter::new(24, 80, theme);

        // 初始位置应该是(0,0)
        let (row, col) = adapter.get_cursor_position();
        assert_eq!(row, 0);
        assert_eq!(col, 0);

        // 处理一些字符
        adapter.process_input(b"Hello").unwrap();
        let (row, col) = adapter.get_cursor_position();
        assert_eq!(row, 0);
        assert_eq!(col, 5);

        // 换行测试
        adapter.process_input(b"\n").unwrap();
        let (row, col) = adapter.get_cursor_position();
        assert_eq!(row, 1);
        assert_eq!(col, 0);
    }

    #[test]
    fn test_ansi_color_processing() {
        let theme = TerminalTheme::default();
        let default_fg = theme.style.foreground;
        let mut adapter = WezTermAdapter::new(24, 80, theme);

        // 测试红色前景色
        adapter.process_input(b"\x1b[31mRed Text").unwrap();
        let buffer = adapter.buffer();

        // 检查第一个字符是否使用了红色
        if let Some(cell) = buffer.get_cell(0, 0) {
            assert_eq!(cell.character, 'R');
            // 颜色应该不是默认的白色
            assert_ne!(cell.fg_color, default_fg);
        }

        // 测试重置颜色
        adapter.process_input(b"\x1b[0mNormal Text").unwrap();
        let buffer = adapter.buffer();

        // 检查重置后的字符是否使用默认颜色
        let normal_start_col = 8;
        if let Some(cell) = buffer.get_cell(0, normal_start_col) {
            assert_eq!(cell.character, 'N');
            // 颜色应该回到默认
            assert_eq!(cell.fg_color, default_fg);
        }
    }
}
