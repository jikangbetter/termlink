//! 终端文本选择处理器
//! 处理鼠标拖拽选择、键盘选择和复制功能

use crate::terminal::buffer::SelectionRange;
use crate::terminal::emulator::TerminalEmulatorTrait;
use copypasta::{ClipboardContext, ClipboardProvider};

/// 选择状态
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionState {
    /// 无选择
    None,
    /// 正在选择中
    Selecting,
    /// 已选择
    Selected,
}

/// 文本选择处理器
pub struct TextSelector {
    /// 当前选择状态
    state: SelectionState,
    /// 选择范围
    selection_range: Option<SelectionRange>,
    /// 选择起始位置（屏幕坐标）
    start_pos: Option<(f32, f32)>,
    /// 当前鼠标位置（屏幕坐标）
    current_pos: Option<(f32, f32)>,
}

impl TextSelector {
    pub fn new() -> Self {
        Self {
            state: SelectionState::None,
            selection_range: None,
            start_pos: None,
            current_pos: None,
        }
    }

    /// 开始选择（鼠标按下）
    pub fn start_selection(&mut self, screen_x: f32, screen_y: f32) {
        self.state = SelectionState::Selecting;
        self.start_pos = Some((screen_x, screen_y));
        self.current_pos = Some((screen_x, screen_y));
        println!("开始选择: ({}, {})", screen_x, screen_y);
    }

    /// 更新选择（鼠标移动）
    pub fn update_selection(&mut self, screen_x: f32, screen_y: f32) {
        if self.state == SelectionState::Selecting {
            self.current_pos = Some((screen_x, screen_y));
            println!("更新选择: ({}, {})", screen_x, screen_y);
        }
    }

    /// 结束选择（鼠标释放）
    pub fn end_selection(&mut self) {
        if self.state == SelectionState::Selecting {
            self.state = SelectionState::Selected;
            println!("结束选择");
        }
    }

    /// 取消选择
    pub fn cancel_selection(&mut self) {
        self.state = SelectionState::None;
        self.selection_range = None;
        self.start_pos = None;
        self.current_pos = None;
    }

    /// 获取选择状态
    pub fn state(&self) -> &SelectionState {
        &self.state
    }

    /// 将屏幕坐标转换为缓冲区坐标
    pub fn screen_to_buffer_coords(
        &self,
        screen_x: f32,
        screen_y: f32,
        rect: &egui::Rect,
        char_size: egui::Vec2,
        line_height: f32,
        buffer_rows: usize,
        buffer_cols: usize,
        history_size: usize,
    ) -> Option<(usize, usize)> {
        if !rect.contains(egui::pos2(screen_x, screen_y)) {
            return None;
        }

        // 计算相对于渲染区域的坐标
        let rel_x = screen_x - rect.min.x;
        let rel_y = screen_y - rect.min.y;

        // 计算总行数（历史+当前屏幕）
        let total_rows = history_size + buffer_rows;

        // 计算行号（从0开始）
        // 渲染时使用的是 char_size.y * line_height，所以这里也要对应
        let row = (rel_y / (char_size.y * line_height)).floor() as usize;
        if row >= total_rows {
            return None;
        }

        // 计算列号（从0开始）
        let col = (rel_x / char_size.x).floor() as usize;
        if col >= buffer_cols {
            return None;
        }

        Some((row, col))
    }

    /// 更新仿真器的选择状态
    pub fn update_emulator_selection(
        &self,
        emulator: &mut dyn TerminalEmulatorTrait,
        rect: &egui::Rect,
        char_size: egui::Vec2,
        line_height: f32,
    ) {
        if self.state != SelectionState::Selecting {
            return;
        }

        if let (Some(start_pos), Some(current_pos)) = (self.start_pos, self.current_pos) {
            // 先获取一次 buffer 只是为了读取行列信息
            let buffer = emulator.buffer();
            if let Some((start_row, start_col)) = self.screen_to_buffer_coords(
                start_pos.0,
                start_pos.1,
                rect,
                char_size,
                line_height,
                buffer.rows,
                buffer.cols,
                buffer.history.len(),
            ) {
                if let Some((end_row, end_col)) = self.screen_to_buffer_coords(
                    current_pos.0,
                    current_pos.1,
                    rect,
                    char_size,
                    line_height,
                    buffer.rows,
                    buffer.cols,
                    buffer.history.len(),
                ) {
                    // 清除之前的选择显示
                    emulator.clear_selection();
                    println!(
                        "选择范围: 从 ({}, {}) 到 ({}, {})",
                        start_row, start_col, end_row, end_col
                    );
                    // 设置新的选择
                    emulator.start_selection(start_row, start_col);
                    emulator.update_selection(end_row, end_col);
                }
            }
        }
    }

    /// 获取选中的文本用于复制
    pub fn get_selected_text_content(
        &self,
        emulator: &dyn TerminalEmulatorTrait,
    ) -> Option<String> {
        emulator.get_selected_text()
    }

    /// 复制选中文本到剪贴板
    pub fn copy_selected_text(&self, emulator: &dyn TerminalEmulatorTrait) -> bool {
        if let Some(text) = self.get_selected_text_content(emulator) {
            match ClipboardContext::new() {
                Ok(mut ctx) => match ctx.set_contents(text) {
                    Ok(_) => {
                        println!("已通过 copypasta 复制到剪贴板");
                        true
                    }
                    Err(_) => false,
                },
                Err(_) => false,
            }
        } else {
            false
        }
    }

    /// 从剪贴板获取文本内容 (粘贴时使用)
    pub fn get_clipboard_text(&self) -> Option<String> {
        match ClipboardContext::new() {
            Ok(mut ctx) => match ctx.get_contents() {
                Ok(text) => {
                    println!("从 copypasta 获取到剪贴板文本, 长度: {}", text.len());
                    Some(text)
                }
                Err(e) => {
                    eprintln!("[TermLink] copypasta 读取剪贴板失败: {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("[TermLink] 初始化 copypasta 失败: {}", e);
                None
            }
        }
    }

    /// 全选
    pub fn select_all(&mut self, emulator: &mut dyn TerminalEmulatorTrait) {
        // 先获取一次 buffer 只为了读取行列信息
        let buffer = emulator.buffer();
        // 选择所有内容：从历史记录开始到当前屏幕结束
        let total_rows = buffer.history.len() + buffer.rows;
        if total_rows > 0 && buffer.cols > 0 {
            emulator.clear_selection();
            emulator.start_selection(0, 0);
            emulator.update_selection(total_rows - 1, buffer.cols - 1);
            self.state = SelectionState::Selected;
        }
    }

    /// 处理键盘快捷键
    pub fn handle_keyboard_shortcuts(
        &mut self,
        emulator: &mut dyn TerminalEmulatorTrait,
        key: &egui::Key,
        modifiers: &egui::Modifiers,
    ) -> bool {
        // Ctrl+A 全选
        if modifiers.ctrl && *key == egui::Key::A {
            self.select_all(emulator);
            return true;
        }

        // Ctrl+Shift+C 复制
        if modifiers.ctrl && modifiers.shift && *key == egui::Key::C {
            return self.copy_selected_text(emulator);
        }

        // Esc 取消选择
        if *key == egui::Key::Escape {
            self.cancel_selection();
            emulator.clear_selection();
            return true;
        }

        false
    }
}

impl Default for TextSelector {
    fn default() -> Self {
        Self::new()
    }
}
