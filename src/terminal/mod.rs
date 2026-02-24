//! 终端仿真器模块
//! 提供SSH终端仿真功能

pub mod buffer;
pub mod emulator;
pub mod renderer;
pub mod theme;
pub mod wezterm_adapter;

// 重新导出主要组件
pub use buffer::{TerminalBuffer, TerminalCell};
pub use emulator::{TerminalEmulator, TerminalEvent};
pub use renderer::TerminalRenderer;
pub use theme::{TerminalTheme, ThemeStyle};
pub use wezterm_adapter::WezTermAdapter;
