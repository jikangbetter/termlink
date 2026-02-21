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

/// 终端设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    pub font_size: f32,
    pub font_family: String,
    pub theme: String,
    pub cursor_blink: bool,
}

/// 外观设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: String,
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
            },
            appearance: AppearanceSettings {
                theme: "dark".to_string(),
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
            let settings: AppSettings = serde_json::from_str(&content)?;
            Ok(settings)
        } else {
            // 返回默认设置
            let settings = AppSettings::default();
            settings.save()?;
            Ok(settings)
        }
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
}
