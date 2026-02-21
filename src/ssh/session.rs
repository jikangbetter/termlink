//! SSH会话管理
//! 管理SSH连接会话和相关状态

use crate::ssh::SshClient;
use crate::ssh::client::SshConfig;
use ssh2::Channel;
use std::sync::{Arc, Mutex};

/// SSH会话状态
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// SSH会话
#[derive(Clone)]
pub struct SshSession {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub state: SessionState,
    pub client: Option<SshClient>,
    /// 终端通道
    pub channel: Arc<Mutex<Option<Channel>>>,
}

impl SshSession {
    /// 创建新的SSH会话
    pub fn new(name: String, host: String, port: u16) -> Self {
        Self {
            name,
            host,
            port,
            state: SessionState::Disconnected,
            client: None,
            channel: Arc::new(Mutex::new(None)),
        }
    }

    /// 连接到远程主机
    pub fn connect(
        &mut self,
        username: &str,
        password: Option<&str>,
        key_path: Option<&str>,
    ) -> anyhow::Result<()> {
        self.state = SessionState::Connecting;

        let config = SshConfig {
            host: self.host.clone(),
            port: self.port,
            username: username.to_string(),
            password: password.map(|s| s.to_string()),
            private_key_path: key_path.map(|s| s.to_string()),
            timeout: Some(30),
        };

        match SshClient::new(config) {
            Ok(client) => {
                // 创建终端通道
                let session = client.session();
                let mut channel = session.channel_session()?;
                channel.request_pty("xterm-256color", None, Some((80, 24, 0, 0)))?;
                channel.shell()?;

                // 设置会话为非阻塞模式，防止UI卡顿
                session.set_blocking(false);

                // 存储通道
                *self.channel.lock().unwrap() = Some(channel);
                self.client = Some(client);
                self.state = SessionState::Connected;
                Ok(())
            }
            Err(e) => {
                self.state = SessionState::Error(e.to_string());
                Err(e)
            }
        }
    }

    /// 断开连接
    pub fn disconnect(&mut self) {
        if let Some(ref client) = self.client {
            // 断开前恢复阻塞模式可能是个好主意，但这里直接丢弃 client 即可
            let _ = client.session().set_blocking(true);
        }
        self.client = None;
        self.state = SessionState::Disconnected;
    }

    /// 检查是否已连接
    pub fn is_connected(&self) -> bool {
        matches!(self.state, SessionState::Connected)
    }

    /// 获取会话状态
    pub fn state(&self) -> &SessionState {
        &self.state
    }

    /// 执行命令（如果已连接）
    pub fn execute_command(&self, command: &str) -> anyhow::Result<String> {
        if let Some(ref client) = self.client {
            client.execute_command(command)
        } else {
            Err(anyhow::anyhow!("Not connected"))
        }
    }

    /// 从终端读取数据
    pub fn read_terminal(&self, buffer: &mut [u8]) -> anyhow::Result<usize> {
        let mut channel_guard = self.channel.lock().unwrap();
        if let Some(ref mut channel) = *channel_guard {
            use std::io::Read;
            match channel.read(buffer) {
                Ok(n) => Ok(n),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(0),
                Err(e) => Err(anyhow::anyhow!("读取失败: {}", e)),
            }
        } else {
            Err(anyhow::anyhow!("No terminal channel"))
        }
    }

    /// 向终端写入数据
    pub fn write_terminal(&self, data: &[u8]) -> anyhow::Result<()> {
        let mut channel_guard = self.channel.lock().unwrap();
        if let Some(ref mut channel) = *channel_guard {
            use std::io::Write;

            // 在非阻塞模式下 manual 循环写，避免 write_all 直接报错
            let mut total_written = 0;
            let mut retries = 0;

            while total_written < data.len() && retries < 100 {
                match channel.write(&data[total_written..]) {
                    Ok(n) if n > 0 => {
                        total_written += n;
                        retries = 0;
                    }
                    Ok(_) => return Err(anyhow::anyhow!("写入0字节")),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // 如果写入失败且数据很少（通常是快捷键），尝试重试几次
                        if data.len() < 10 {
                            std::thread::sleep(std::time::Duration::from_millis(1));
                            retries += 1;
                            continue;
                        }

                        if total_written > 0 {
                            break;
                        } else {
                            // 让 UI 继续运行，下次重试
                            return Ok(());
                        }
                    }
                    Err(e) => return Err(anyhow::anyhow!("写入失败: {}", e)),
                }
            }
            let _ = channel.flush();
            Ok(())
        } else {
            Err(anyhow::anyhow!("No terminal channel"))
        }
    }

    /// 检查终端通道是否可读
    pub fn is_terminal_readable(&self) -> bool {
        let channel_guard = self.channel.lock().unwrap();
        // 如果通道正在进行某些内部操作，且为非阻塞模式，这里可以总是返回 true，
        // 由 read_terminal 的已修改逻辑来处理 WouldBlock。
        channel_guard.is_some()
    }

    /// 调整终端窗口大小
    pub fn resize_terminal(&self, rows: u32, cols: u32) -> anyhow::Result<()> {
        let mut channel_guard = self.channel.lock().unwrap();
        if let Some(ref mut channel) = *channel_guard {
            channel.request_pty_size(cols, rows, Some(0), Some(0))?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No terminal channel"))
        }
    }
}
