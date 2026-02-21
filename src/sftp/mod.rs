//! SFTP文件管理模块
//! 处理文件浏览、上传下载等SFTP功能

pub mod manager;

// 重新导出主要组件
pub use manager::SftpManager;
