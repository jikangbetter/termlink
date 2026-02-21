//! SSH客户端实现
//! 处理SSH连接的建立和管理

use anyhow::Result;
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;

/// SSH客户端配置
#[derive(Debug, Clone)]
pub struct SshConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub private_key_path: Option<String>,
    pub timeout: Option<u64>,
}

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 22,
            username: "user".to_string(),
            password: None,
            private_key_path: None,
            timeout: Some(30),
        }
    }
}

/// SSH客户端
#[derive(Clone)]
pub struct SshClient {
    session: Session,
    config: SshConfig,
}

impl SshClient {
    /// 创建新的SSH客户端
    pub fn new(config: SshConfig) -> Result<Self> {
        let tcp = TcpStream::connect((config.host.as_str(), config.port))?;

        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;

        // 认证
        if let Some(ref key_path) = config.private_key_path {
            session.userauth_pubkey_file(&config.username, None, Path::new(key_path), None)?;
        } else if let Some(ref password) = config.password {
            session.userauth_password(&config.username, password)?;
        } else {
            return Err(anyhow::anyhow!("No authentication method provided"));
        }

        if !session.authenticated() {
            return Err(anyhow::anyhow!("Authentication failed"));
        }

        Ok(Self { session, config })
    }

    /// 检查连接是否已认证
    pub fn is_authenticated(&self) -> bool {
        self.session.authenticated()
    }

    /// 获取底层SSH会话
    pub fn session(&self) -> &Session {
        &self.session
    }

    /// 获取配置信息
    pub fn config(&self) -> &SshConfig {
        &self.config
    }

    /// 执行远程命令
    pub fn execute_command(&self, command: &str) -> Result<String> {
        let mut channel = self.session.channel_session()?;
        channel.exec(command)?;

        let mut output = String::new();
        channel.read_to_string(&mut output)?;
        channel.wait_close()?;

        Ok(output)
    }
}
