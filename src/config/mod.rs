//! 配置管理模块
//! 处理应用配置、用户设置和连接配置

pub mod settings;

// 重新导出主要组件
pub use settings::{AppSettings, ConnectionConfig};
