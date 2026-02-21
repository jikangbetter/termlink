//! 工具函数集合
//! 提供各种辅助功能
#![allow(unused)]

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
    port > 0
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

/// 获取字符在终端中的显示宽度 (CJK)
pub fn get_char_width(c: char) -> usize {
    if (c >= '\u{1100}' && c <= '\u{115f}') || // 谚文
       (c >= '\u{2e80}' && c <= '\u{9fff}') || // 常用中日韩汉字
       (c >= '\u{ac00}' && c <= '\u{d7a3}') || // 韩文音节
       (c >= '\u{f900}' && c <= '\u{faff}') || // CJK 兼容汉字
       (c >= '\u{fe10}' && c <= '\u{fe19}') || // 垂直形式
       (c >= '\u{fe30}' && c <= '\u{fe6f}') || // CJK 兼容性形式
       (c >= '\u{ff00}' && c <= '\u{ff60}') || // 全角 ASCII 变体
       (c >= '\u{ffe0}' && c <= '\u{ffe6}') || // 全角标点
       (c >= '\u{20000}' && c <= '\u{2fffd}') || // 扩展 B-F
       (c >= '\u{30000}' && c <= '\u{3fffd}')
    // 扩展 G
    {
        2
    } else {
        1
    }
}

/// 检测系统主题设置
pub fn detect_system_theme() -> String {
    #[cfg(windows)]
    {
        // Windows 系统主题检测
        use std::process::Command;

        let output = Command::new("reg")
            .args([
                "query",
                "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output()
            .ok();

        if let Some(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("0x0") {
                    return "dark".to_string();
                } else {
                    return "light".to_string();
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 系统主题检测
        use std::process::Command;

        // 检查 GNOME
        let output = Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output()
            .ok();

        if let Some(output) = output {
            if output.status.success() {
                let theme_name = String::from_utf8_lossy(&output.stdout);
                if theme_name.contains("dark") || theme_name.contains("Dark") {
                    return "dark".to_string();
                } else {
                    return "light".to_string();
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS 系统主题检测
        use std::process::Command;

        let output = Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
            .ok();

        if let Some(output) = output {
            if output.status.success() {
                return "dark".to_string();
            }
        }
        return "light".to_string();
    }

    // 默认返回深色主题
    "dark".to_string()
}
