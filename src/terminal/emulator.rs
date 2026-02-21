//! 专业终端仿真器实现
//! 使用vte库实现VT100/VT220终端仿真

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
    buffer: String,
    /// 事件回调
    callback: Option<Box<dyn Fn(TerminalEvent) + Send + Sync>>,
}

impl VtePerform {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            callback: None,
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
        self.buffer.push(c);
        self.send_event(TerminalEvent::Output(c.to_string()));
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            0x08 => {
                // Backspace
                if !self.buffer.is_empty() {
                    self.buffer.pop();
                    self.send_event(TerminalEvent::Output("\x08 \x08".to_string()));
                }
            }
            0x0a => {
                // Line feed
                self.buffer.push('\n');
                self.send_event(TerminalEvent::Output("\n".to_string()));
            }
            0x0d => {
                // Carriage return
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
        _params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        c: char,
    ) {
        // 处理CSI控制序列
        match c {
            'H' | 'f' => {
                // 光标定位
                self.send_event(TerminalEvent::CursorPosition { row: 1, col: 1 });
            }
            'J' => {
                // 清屏
                self.send_event(TerminalEvent::Output("\x1b[2J\x1b[H".to_string()));
            }
            'm' => {
                // 颜色和样式
                // 处理ANSI颜色代码
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
        let performer = VtePerform::new();

        Self {
            parser: Parser::new(),
            performer: Arc::new(Mutex::new(performer)),
            state: TerminalState::Disconnected,
        }
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
        self.performer.lock().unwrap().buffer.clear();
    }

    /// 调整终端大小
    pub fn resize(&mut self, rows: usize, cols: usize) {
        if let Some(ref callback) = self.performer.lock().unwrap().callback {
            callback(TerminalEvent::Resize { rows, cols });
        }
    }
}
