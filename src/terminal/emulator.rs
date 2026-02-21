//! 专业终端仿真器实现
//! 使用vte库实现VT100/VT220终端仿真

use crate::terminal::buffer::TerminalBuffer;
use crate::terminal::theme::TerminalTheme;
use std::sync::{Arc, Mutex};
use vte::{Parser, Perform};

/// 终端事件回调
#[derive(Debug, Clone)]
pub enum TerminalEvent {
    /// 输出内容更新
    Output(String),
    /// 光标位置改变
    CursorPosition { row: usize, col: usize },
    /// 终端大小改变
    Resize { rows: usize, cols: usize },
    /// 请求用户输入
    RequestInput,
    /// 终端标题改变
    TitleChange(String),
}

/// 终端状态
#[derive(Debug, Clone, PartialEq)]
pub enum TerminalState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// VTE性能实现
struct VtePerform {
    /// 终端缓冲区
    pub term_buffer: TerminalBuffer,
    /// 终端主题（用于颜色映射）
    pub theme: TerminalTheme,
    /// 旧的字符串缓冲区（用于向后兼容）
    pub buffer: String,
    /// 事件回调
    callback: Option<Box<dyn Fn(TerminalEvent) + Send + Sync>>,
    /// 当前前景色
    current_fg: Option<u8>,
    /// 当前背景色
    current_bg: Option<u8>,
    /// 是否加粗
    bold: bool,
}

impl VtePerform {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            term_buffer: TerminalBuffer::new(rows, cols),
            theme: TerminalTheme::default(),
            buffer: String::new(),
            callback: None,
            current_fg: None,
            current_bg: None,
            bold: false,
        }
    }

    fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(TerminalEvent) + Send + Sync + 'static,
    {
        self.callback = Some(Box::new(callback));
    }

    fn send_event(&self, event: TerminalEvent) {
        if let Some(ref callback) = self.callback {
            callback(event);
        }
    }
}

impl Perform for VtePerform {
    fn print(&mut self, c: char) {
        // 获取字符宽度
        let width = crate::utils::helpers::get_char_width(c);

        // 更新旧的字符串缓冲区
        self.buffer.push(c);

        // 更新单元格缓冲区
        let row = self.term_buffer.cursor_row;
        let col = self.term_buffer.cursor_col;

        if let Some(cell) = self.term_buffer.get_cell_mut(row, col) {
            cell.character = c;
            cell.fg_color = if let Some(fg) = self.current_fg {
                self.theme.get_color(fg, self.bold)
            } else {
                self.theme.style.foreground
            };
            cell.bg_color = if let Some(bg) = self.current_bg {
                self.theme.get_color(bg, false)
            } else {
                egui::Color32::TRANSPARENT
            };
            cell.bold = self.bold;
            cell.is_continuation = false;
        }

        // 如果是宽字符且后面还有位置，标记下一格为延续位
        if width == 2 && col < self.term_buffer.cols - 1 {
            if let Some(next_cell) = self.term_buffer.get_cell_mut(row, col + 1) {
                next_cell.character = ' ';
                next_cell.is_continuation = true;
            }
        }

        // 移动光标
        let move_cols = width;
        if col + move_cols < self.term_buffer.cols {
            self.term_buffer.cursor_col += move_cols;
        } else {
            self.term_buffer.newline();
            self.term_buffer.cursor_col = 0;
        }

        self.send_event(TerminalEvent::Output(c.to_string()));
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            0x08 | 0x7f => {
                // Backspace (0x08) 或者 Delete (0x7f)
                // 大多数现代系统将 0x7f 作为退格处理
                self.term_buffer.backspace();
            }
            0x0a => {
                // Line feed
                self.term_buffer.newline();
                self.buffer.push('\n');
                self.send_event(TerminalEvent::Output("\n".to_string()));
            }
            0x0d => {
                // Carriage return
                self.term_buffer.carriage_return();
                // 不添加到缓冲区，但触发回调
                self.send_event(TerminalEvent::Output("\r".to_string()));
            }
            _ => {
                // 其他控制字符
                self.send_event(TerminalEvent::Output(format!("{}", byte as char)));
            }
        }
    }

    fn hook(&mut self, _params: &vte::Params, _intermediates: &[u8], _ignore: bool, _c: char) {
        // 处理CSI序列开始
    }

    fn put(&mut self, _byte: u8) {
        // 处理CSI序列中的字节
    }

    fn unhook(&mut self) {
        // 处理CSI序列结束
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
        // 处理OSC序列
        if params.len() > 0 {
            if params[0] == b"0" || params[0] == b"2" {
                if params.len() > 1 {
                    let title = String::from_utf8_lossy(params[1]);
                    self.send_event(TerminalEvent::TitleChange(title.to_string()));
                }
            }
        }
    }

    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        c: char,
    ) {
        // 处理CSI控制序列
        match c {
            'H' | 'f' => {
                // 光标定位 (CSI Row;Col H/f)
                let mut it = params.iter();
                let row = it
                    .next()
                    .and_then(|p| p.get(0))
                    .map(|&v| if v > 0 { v as usize } else { 1 })
                    .unwrap_or(1);
                let col = it
                    .next()
                    .and_then(|p| p.get(0))
                    .map(|&v| if v > 0 { v as usize } else { 1 })
                    .unwrap_or(1);

                self.term_buffer
                    .set_cursor(row.saturating_sub(1), col.saturating_sub(1));
                self.send_event(TerminalEvent::CursorPosition { row, col });
            }
            'A' => {
                // 光标上移
                let n = params.iter().next().and_then(|p| p.get(0)).unwrap_or(&1);
                let new_row = self.term_buffer.cursor_row.saturating_sub(*n as usize);
                self.term_buffer.cursor_row = new_row;
            }
            'B' => {
                // 光标下移
                let n = params.iter().next().and_then(|p| p.get(0)).unwrap_or(&1);
                let new_row =
                    (self.term_buffer.cursor_row + *n as usize).min(self.term_buffer.rows - 1);
                self.term_buffer.cursor_row = new_row;
            }
            'C' => {
                // 光标右移
                let n = params.iter().next().and_then(|p| p.get(0)).unwrap_or(&1);
                let new_col =
                    (self.term_buffer.cursor_col + *n as usize).min(self.term_buffer.cols - 1);
                self.term_buffer.cursor_col = new_col;
            }
            'D' => {
                // 光标左移
                let n = params.iter().next().and_then(|p| p.get(0)).unwrap_or(&1);
                let new_col = self.term_buffer.cursor_col.saturating_sub(*n as usize);
                self.term_buffer.cursor_col = new_col;
            }
            'K' => {
                // 清除行 (CSI n K)
                let mode = params.iter().next().and_then(|p| p.get(0)).unwrap_or(&0);
                let row = self.term_buffer.cursor_row;
                match mode {
                    0 => {
                        // 从光标清除到行尾
                        for col in self.term_buffer.cursor_col..self.term_buffer.cols {
                            if let Some(cell) = self.term_buffer.get_cell_mut(row, col) {
                                *cell = crate::terminal::buffer::TerminalCell::default();
                            }
                        }
                    }
                    1 => {
                        // 从行首清除到光标
                        for col in 0..=self.term_buffer.cursor_col {
                            if let Some(cell) = self.term_buffer.get_cell_mut(row, col) {
                                *cell = crate::terminal::buffer::TerminalCell::default();
                            }
                        }
                    }
                    2 => {
                        // 清除整行
                        for col in 0..self.term_buffer.cols {
                            if let Some(cell) = self.term_buffer.get_cell_mut(row, col) {
                                *cell = crate::terminal::buffer::TerminalCell::default();
                            }
                        }
                    }
                    _ => {}
                }
            }
            'J' => {
                // 清屏
                let mode = params.iter().next().and_then(|p| p.get(0)).unwrap_or(&0);
                match mode {
                    2 | 3 => {
                        // 全屏清除
                        self.term_buffer.clear();
                    }
                    _ => {
                        // 暂时简略处理其他模式
                        self.term_buffer.clear();
                    }
                }
                self.send_event(TerminalEvent::Output("\x1b[2J\x1b[H".to_string()));
            }
            'm' => {
                // 颜色和样式
                // 处理ANSI颜色代码
                if params.len() == 0 {
                    // 重置所有属性
                    self.current_fg = None;
                    self.current_bg = None;
                    self.bold = false;
                } else {
                    // 处理颜色参数
                    for param in params.iter() {
                        if param.len() > 0 {
                            match param[0] {
                                0 => {
                                    // 重置
                                    self.current_fg = None;
                                    self.current_bg = None;
                                    self.bold = false;
                                }
                                1 => {
                                    // 加粗
                                    self.bold = true;
                                }
                                30..=37 => {
                                    // 前景色
                                    self.current_fg = Some((param[0] - 30) as u8);
                                }
                                40..=47 => {
                                    // 背景色
                                    self.current_bg = Some((param[0] - 40) as u8);
                                }
                                90..=97 => {
                                    // 亮前景色
                                    self.current_fg = Some((param[0] - 90 + 8) as u8);
                                }
                                100..=107 => {
                                    // 亮背景色
                                    self.current_bg = Some((param[0] - 100 + 8) as u8);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                self.send_event(TerminalEvent::Output("\x1b[m".to_string()));
            }
            _ => {
                // 其他CSI序列
            }
        }
    }
}

/// 专业终端仿真器
pub struct TerminalEmulator {
    /// VTE解析器
    parser: Parser,
    /// VTE处理器
    performer: Arc<Mutex<VtePerform>>,
    /// 终端状态
    state: TerminalState,
}

impl TerminalEmulator {
    /// 创建新的终端仿真器
    pub fn new(rows: usize, cols: usize) -> Self {
        let performer = VtePerform::new(rows, cols);

        Self {
            parser: Parser::new(),
            performer: Arc::new(Mutex::new(performer)),
            state: TerminalState::Disconnected,
        }
    }

    /// 更新主题
    pub fn update_theme(&self, theme: TerminalTheme) {
        let mut performer = self.performer.lock().unwrap();
        performer.theme = theme;
    }

    /// 获取终端缓冲区
    pub fn buffer(&self) -> TerminalBuffer {
        self.performer.lock().unwrap().term_buffer.clone()
    }

    /// 设置事件回调
    pub fn set_event_callback<F>(&mut self, callback: F)
    where
        F: Fn(TerminalEvent) + Send + Sync + 'static,
    {
        self.performer.lock().unwrap().set_callback(callback);
    }

    /// 处理输入数据
    pub fn process_input(&mut self, data: &[u8]) -> anyhow::Result<()> {
        let mut performer = self.performer.lock().unwrap();
        self.parser.advance(&mut *performer, data);
        Ok(())
    }

    /// 发送输出到终端
    pub fn send_output(&mut self, output: &str) {
        let mut performer = self.performer.lock().unwrap();
        self.parser.advance(&mut *performer, output.as_bytes());
    }

    /// 获取终端缓冲区内容
    pub fn get_buffer(&self) -> String {
        self.performer.lock().unwrap().buffer.clone()
    }

    /// 设置终端状态
    pub fn set_state(&mut self, state: TerminalState) {
        self.state = state;
    }

    /// 获取终端状态
    pub fn state(&self) -> &TerminalState {
        &self.state
    }

    /// 清屏
    pub fn clear(&mut self) {
        let mut perf = self.performer.lock().unwrap();
        perf.buffer.clear();
        perf.term_buffer.clear();
    }

    /// 调整终端大小
    pub fn resize(&mut self, rows: usize, cols: usize) {
        let mut perf = self.performer.lock().unwrap();
        perf.term_buffer.resize(rows, cols);
        if let Some(ref callback) = perf.callback {
            callback(TerminalEvent::Resize { rows, cols });
        }
    }
}
