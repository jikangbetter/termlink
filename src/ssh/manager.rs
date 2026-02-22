//! SSH连接管理器
//! 统一管理SSH连接、配置和会话

use crate::config::ConnectionConfig;
use crate::ssh::{SessionState, SshClient, SshConfig, SshSession};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 会话信息（用于UI显示）
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub state: SessionState,
}

/// 连接管理器
pub struct ConnectionManager {
    /// 活动会话
    sessions: Arc<Mutex<HashMap<String, SshSession>>>,
    /// 连接历史配置
    connection_configs: Vec<ConnectionConfig>,
}

impl ConnectionManager {
    /// 创建新的连接管理器
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            connection_configs: Vec::new(),
        }
    }

    /// 从配置加载连接管理器
    pub fn from_configs(configs: Vec<ConnectionConfig>) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            connection_configs: configs,
        }
    }

    /// 添加连接配置
    pub fn add_connection_config(&mut self, config: ConnectionConfig) {
        // 检查是否已存在同名配置
        if let Some(existing) = self
            .connection_configs
            .iter_mut()
            .find(|c| c.name == config.name)
        {
            *existing = config; // 更新现有配置
        } else {
            self.connection_configs.push(config); // 添加新配置
        }
    }

    /// 获取所有连接配置
    pub fn get_connection_configs(&self) -> &[ConnectionConfig] {
        &self.connection_configs
    }

    /// 添加会话
    pub fn add_session(&self, name: String, session: SshSession) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(name, session);
    }

    /// 根据名称获取连接配置
    pub fn get_connection_config(&self, name: &str) -> Option<&ConnectionConfig> {
        self.connection_configs.iter().find(|c| c.name == name)
    }

    /// 删除连接配置
    pub fn remove_connection_config(&mut self, name: &str) -> bool {
        let initial_len = self.connection_configs.len();
        self.connection_configs.retain(|c| c.name != name);
        initial_len > self.connection_configs.len()
    }

    /// 创建并连接到SSH会话
    pub fn connect(&self, config: &ConnectionConfig, password: Option<&str>) -> Result<String> {
        let session_name = config.name.clone();

        // 创建处于连接中状态的会话并立即存入
        let mut session = SshSession::new(session_name.clone(), config.host.clone(), config.port);
        session.state = crate::ssh::SessionState::Connecting;

        {
            let mut sessions = self.sessions.lock().unwrap();
            sessions.insert(session_name.clone(), session.clone());
        }

        // 实际连接（如果是阻塞调用，则在当前线程执行；如果是异步调用，已在外部包装为线程）
        match session.connect(
            &config.username,
            password,
            config.private_key_path.as_deref(),
        ) {
            Ok(_) => {
                // 连接成功，更新管理器中的会话
                let mut sessions = self.sessions.lock().unwrap();
                sessions.insert(session_name.clone(), session);
                Ok(session_name)
            }
            Err(e) => {
                // 连接失败，更新状态为错误
                let mut sessions = self.sessions.lock().unwrap();
                if let Some(s) = sessions.get_mut(&session_name) {
                    s.state = crate::ssh::SessionState::Error(e.to_string());
                }
                Err(e)
            }
        }
    }

    /// 断开会话连接
    pub fn disconnect(&self, session_name: &str) -> bool {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(mut session) = sessions.remove(session_name) {
            session.disconnect();
            true
        } else {
            false
        }
    }

    /// 获取会话（可变引用）
    pub fn get_session(&self, session_name: &str) -> Option<SshSession> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.get_mut(session_name).cloned()
    }

    /// 获取会话状态信息（不包含客户端）
    pub fn get_session_info(&self, session_name: &str) -> Option<SessionInfo> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(session_name).map(|s| SessionInfo {
            name: s.name.clone(),
            host: s.host.clone(),
            port: s.port,
            state: s.state.clone(),
        })
    }

    /// 获取所有活动会话名称
    pub fn get_active_sessions(&self) -> Vec<String> {
        let sessions = self.sessions.lock().unwrap();
        sessions.keys().cloned().collect()
    }

    /// 检查会话是否活跃
    pub fn is_session_active(&self, session_name: &str) -> bool {
        let sessions = self.sessions.lock().unwrap();
        sessions.contains_key(session_name)
    }

    /// 检查会话是否存在并已连接
    pub fn has_session(&self, session_name: &str) -> bool {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_name) {
            session.is_connected()
        } else {
            false
        }
    }

    /// 执行远程命令
    pub fn execute_command(&self, session_name: &str, command: &str) -> Result<String> {
        let sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get(session_name) {
            session.execute_command(command)
        } else {
            Err(anyhow::anyhow!("会话 {} 不存在", session_name))
        }
    }

    /// 获取会话状态
    pub fn get_session_state(&self, session_name: &str) -> Option<String> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(session_name).map(|s| match s.state() {
            crate::ssh::SessionState::Disconnected => "已断开".to_string(),
            crate::ssh::SessionState::Connecting => "连接中".to_string(),
            crate::ssh::SessionState::Connected => "已连接".to_string(),
            crate::ssh::SessionState::Error(msg) => format!("错误: {}", msg),
        })
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 连接测试结果
#[derive(Debug)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub message: String,
    pub latency: Option<u64>, // 延迟（毫秒）
}

/// 测试SSH连接
pub fn test_connection(config: &ConnectionConfig) -> ConnectionTestResult {
    use std::time::Instant;

    let start_time = Instant::now();

    // 创建测试配置
    let ssh_config = SshConfig {
        host: config.host.clone(),
        port: config.port,
        username: config.username.clone(),
        password: if config.use_key_auth {
            None
        } else {
            Some("".to_string()) // 测试时使用空密码
        },
        private_key_path: config.private_key_path.clone(),
        timeout: Some(10), // 10秒超时
    };

    match SshClient::new(ssh_config) {
        Ok(_) => {
            let latency = start_time.elapsed().as_millis() as u64;
            ConnectionTestResult {
                success: true,
                message: "连接成功".to_string(),
                latency: Some(latency),
            }
        }
        Err(e) => ConnectionTestResult {
            success: false,
            message: format!("连接失败: {}", e),
            latency: None,
        },
    }
}
