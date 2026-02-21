//! SSH相关功能模块
//! 处理SSH连接、会话管理和终端仿真

pub mod client;
pub mod manager;
pub mod session;

// 重新导出主要组件
pub use client::{SshClient, SshConfig};
pub use manager::{ConnectionManager, ConnectionTestResult, test_connection};
pub use session::{SessionState, SshSession};
