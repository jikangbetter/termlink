//! 应用设置管理
//! 处理配置文件的读写和管理

use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 窗口设置
    pub window: WindowSettings,
    /// 连接历史
    pub connections: Vec<ConnectionConfig>,
    /// 连接分组
    pub groups: Vec<ConnectionGroup>,
    /// 终端设置
    pub terminal: TerminalSettings,
    /// 外观设置
    pub appearance: AppearanceSettings,
}

/// 窗口设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSettings {
    pub width: f32,
    pub height: f32,
    pub maximized: bool,
}

/// 连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub use_key_auth: bool,
    pub private_key_path: Option<String>,
    pub password: Option<String>, // 保存的密码
    pub last_connected: Option<String>,
    pub group: Option<String>, // 所属分组
}

/// 连接分组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionGroup {
    pub name: String,
    pub description: Option<String>,
    pub connections: Vec<String>, // 连接名称列表
}

/// 自定义主题颜色配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTheme {
    pub foreground: String, // 前景色 (RGB格式: "#RRGGBB")
    pub background: String, // 背景色
    pub cursor: String,     // 光标颜色
    pub selection: String,  // 选择背景色
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

impl CustomTheme {
    /// 获取内置配色预设
    pub fn presets() -> Vec<(&'static str, Self)> {
        vec![
            (
                "Solarized Dark",
                Self {
                    foreground: "#839496".to_string(),
                    background: "#002B36".to_string(),
                    cursor: "#93A1A1".to_string(),
                    selection: "#073642B4".to_string(),
                    black: "#073642".to_string(),
                    red: "#DC322F".to_string(),
                    green: "#859900".to_string(),
                    yellow: "#B58900".to_string(),
                    blue: "#268BD2".to_string(),
                    magenta: "#D33682".to_string(),
                    cyan: "#2AA198".to_string(),
                    white: "#EEE8D5".to_string(),
                    bright_black: "#002B36".to_string(),
                    bright_red: "#CB4B16".to_string(),
                    bright_green: "#586E75".to_string(),
                    bright_yellow: "#657B83".to_string(),
                    bright_blue: "#839496".to_string(),
                    bright_magenta: "#6C71C4".to_string(),
                    bright_cyan: "#93A1A1".to_string(),
                    bright_white: "#FDF6E3".to_string(),
                },
            ),
            (
                "Monokai",
                Self {
                    foreground: "#F8F8F2".to_string(),
                    background: "#272822".to_string(),
                    cursor: "#F8F8F2".to_string(),
                    selection: "#49483EB4".to_string(),
                    black: "#272822".to_string(),
                    red: "#F92672".to_string(),
                    green: "#A6E22E".to_string(),
                    yellow: "#E6DB74".to_string(),
                    blue: "#66D9EF".to_string(),
                    magenta: "#AE81FF".to_string(),
                    cyan: "#A1EFE4".to_string(),
                    white: "#F8F8F2".to_string(),
                    bright_black: "#75715E".to_string(),
                    bright_red: "#F92672".to_string(),
                    bright_green: "#A6E22E".to_string(),
                    bright_yellow: "#E6DB74".to_string(),
                    bright_blue: "#66D9EF".to_string(),
                    bright_magenta: "#AE81FF".to_string(),
                    bright_cyan: "#A1EFE4".to_string(),
                    bright_white: "#F8F8F2".to_string(),
                },
            ),
            (
                "Nord",
                Self {
                    foreground: "#D8DEE9".to_string(),
                    background: "#2E3440".to_string(),
                    cursor: "#D8DEE9".to_string(),
                    selection: "#434C5EB4".to_string(),
                    black: "#3B4252".to_string(),
                    red: "#BF616A".to_string(),
                    green: "#A3BE8C".to_string(),
                    yellow: "#EBCB8B".to_string(),
                    blue: "#81A1C1".to_string(),
                    magenta: "#B48EAD".to_string(),
                    cyan: "#88C0D0".to_string(),
                    white: "#E5E9F0".to_string(),
                    bright_black: "#4C566A".to_string(),
                    bright_red: "#BF616A".to_string(),
                    bright_green: "#A3BE8C".to_string(),
                    bright_yellow: "#EBCB8B".to_string(),
                    bright_blue: "#81A1C1".to_string(),
                    bright_magenta: "#B48EAD".to_string(),
                    bright_cyan: "#8FBCBB".to_string(),
                    bright_white: "#ECEFF4".to_string(),
                },
            ),
            (
                "One Half Dark",
                Self {
                    foreground: "#DCDFE4".to_string(),
                    background: "#282C34".to_string(),
                    cursor: "#A3B3CC".to_string(),
                    selection: "#474E5DB4".to_string(),
                    black: "#282C34".to_string(),
                    red: "#E06C75".to_string(),
                    green: "#98C379".to_string(),
                    yellow: "#E5C07B".to_string(),
                    blue: "#61AFEF".to_string(),
                    magenta: "#C678DD".to_string(),
                    cyan: "#56B6C2".to_string(),
                    white: "#DCDFE4".to_string(),
                    bright_black: "#5C6370".to_string(),
                    bright_red: "#E06C75".to_string(),
                    bright_green: "#98C379".to_string(),
                    bright_yellow: "#E5C07B".to_string(),
                    bright_blue: "#61AFEF".to_string(),
                    bright_magenta: "#C678DD".to_string(),
                    bright_cyan: "#56B6C2".to_string(),
                    bright_white: "#DCDFE4".to_string(),
                },
            ),
        ]
    }
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self {
            foreground: "#FFFFFF".to_string(),
            background: "#0F0F0F".to_string(),
            cursor: "#FFFFFF".to_string(),
            selection: "#6495EDB4".to_string(), // 带alpha的蓝色
            black: "#000000".to_string(),
            red: "#CD3131".to_string(),
            green: "#0DBC79".to_string(),
            yellow: "#E5E510".to_string(),
            blue: "#2472C8".to_string(),
            magenta: "#BC3FBC".to_string(),
            cyan: "#11A8CD".to_string(),
            white: "#E5E5E5".to_string(),
            bright_black: "#666666".to_string(),
            bright_red: "#F14C4C".to_string(),
            bright_green: "#23D18B".to_string(),
            bright_yellow: "#F5F543".to_string(),
            bright_blue: "#3B8EEA".to_string(),
            bright_magenta: "#D670D6".to_string(),
            bright_cyan: "#29B8DB".to_string(),
            bright_white: "#E5E5E5".to_string(),
        }
    }
}

/// 终端设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    pub font_size: f32,
    pub font_family: String,
    pub theme: String,
    pub cursor_blink: bool,
    /// 自定义主题颜色配置
    pub custom_theme: Option<CustomTheme>,
}

/// 主题模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeMode {
    /// 跟随系统主题
    #[serde(rename = "auto")]
    Auto,
    /// 深色主题
    #[serde(rename = "dark")]
    Dark,
    /// 浅色主题
    #[serde(rename = "light")]
    Light,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Auto
    }
}

/// 外观设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme_mode: ThemeMode,
    /// 本地强制的主题（不存储自动获取的状态）
    pub system_theme: String,
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            window: WindowSettings {
                width: 1200.0,
                height: 800.0,
                maximized: false,
            },
            connections: Vec::new(),
            groups: Vec::new(),
            terminal: TerminalSettings {
                font_size: 14.0,
                font_family: "Consolas".to_string(),
                theme: "dark".to_string(),
                cursor_blink: true,
                custom_theme: None,
            },
            appearance: AppearanceSettings {
                theme_mode: ThemeMode::Auto,
                system_theme: "dark".to_string(),
                language: "zh-CN".to_string(),
            },
        }
    }
}

impl AppSettings {
    /// 获取配置文件路径
    fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "TermLink", "TermLink")
            .ok_or_else(|| anyhow::anyhow!("无法获取项目目录"))?;

        let config_dir = proj_dirs.config_dir();
        std::fs::create_dir_all(config_dir)?;

        Ok(config_dir.join("config.json"))
    }

    /// 加载设置
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;

            // 尝试解析新格式
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                Ok(settings)
            } else {
                // 如果失败，尝试解析旧格式并迁移
                Self::migrate_from_old_format(&content)
            }
        } else {
            // 返回默认设置
            let settings = AppSettings::default();
            settings.save()?;
            Ok(settings)
        }
    }

    /// 从旧格式迁移配置
    fn migrate_from_old_format(content: &str) -> Result<Self> {
        // 定义旧格式结构
        #[derive(Deserialize)]
        struct OldAppearanceSettings {
            theme: String,
            language: String,
        }

        #[derive(Deserialize)]
        struct OldAppSettings {
            window: WindowSettings,
            connections: Vec<ConnectionConfig>,
            groups: Vec<ConnectionGroup>,
            terminal: TerminalSettings,
            appearance: OldAppearanceSettings,
        }

        let old_settings: OldAppSettings = serde_json::from_str(content)?;

        // 转换为新格式
        let new_settings = AppSettings {
            window: old_settings.window,
            connections: old_settings.connections,
            groups: old_settings.groups,
            terminal: old_settings.terminal,
            appearance: AppearanceSettings {
                theme_mode: match old_settings.appearance.theme.as_str() {
                    "dark" => ThemeMode::Dark,
                    "light" => ThemeMode::Light,
                    _ => ThemeMode::Auto,
                },
                system_theme: "dark".to_string(), // 默认值
                language: old_settings.appearance.language,
            },
        };

        // 保存新格式
        new_settings.save()?;
        Ok(new_settings)
    }

    /// 保存设置
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }

    /// 添加连接配置
    pub fn add_connection(&mut self, config: ConnectionConfig) {
        self.connections.push(config);
    }

    /// 获取连接配置
    pub fn get_connection(&self, name: &str) -> Option<&ConnectionConfig> {
        self.connections.iter().find(|c| c.name == name)
    }

    /// 删除连接配置
    pub fn remove_connection(&mut self, name: &str) -> bool {
        let initial_len = self.connections.len();
        self.connections.retain(|c| c.name != name);
        self.connections.len() < initial_len
    }

    /// 获取当前应使用的主题
    pub fn get_current_theme(&self) -> String {
        if self.terminal.theme == "custom" {
            return "custom".to_string();
        }
        match self.appearance.theme_mode {
            ThemeMode::Auto => self.appearance.system_theme.clone(),
            ThemeMode::Dark => "dark".to_string(),
            ThemeMode::Light => "light".to_string(),
        }
    }

    /// 获取主题模式的显示名称（需要传入i18n管理器）
    pub fn get_theme_mode_display(&self, i18n: &crate::i18n::I18nManager) -> String {
        match self.appearance.theme_mode {
            ThemeMode::Auto => i18n.get(crate::i18n::I18nKey::AutoTheme).to_string(),
            ThemeMode::Dark => i18n.get(crate::i18n::I18nKey::DarkTheme).to_string(),
            ThemeMode::Light => i18n.get(crate::i18n::I18nKey::LightTheme).to_string(),
        }
    }
}
