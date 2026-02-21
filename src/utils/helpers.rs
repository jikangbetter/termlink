//! 工具函数集合
//! 提供各种辅助功能

use std::path::Path;

/// 验证主机地址格式
pub fn validate_host(host: &str) -> bool {
    !host.is_empty()
        && host
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_')
}

/// 验证端口号
pub fn validate_port(port: u16) -> bool {
    port > 0 && port <= 65535
}

/// 验证用户名
pub fn validate_username(username: &str) -> bool {
    !username.is_empty() && username.len() <= 100
}

/// 检查文件是否存在
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// 获取文件扩展名
pub fn get_file_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}

/// 格式化文件大小
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// 生成时间戳
pub fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    now.to_string()
}

/// 安全的字符串截断
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.chars().count() <= max_length {
        s.to_string()
    } else {
        s.chars().take(max_length - 3).collect::<String>() + "..."
    }
}

/// 检查是否为目录路径
pub fn is_directory_path(path: &str) -> bool {
    path.ends_with('/') || path.ends_with('\\')
}

/// 合并路径
pub fn join_paths(base: &str, relative: &str) -> String {
    if base.is_empty() {
        relative.to_string()
    } else if relative.is_empty() {
        base.to_string()
    } else {
        format!(
            "{}/{}",
            base.trim_end_matches('/'),
            relative.trim_start_matches('/')
        )
    }
}
