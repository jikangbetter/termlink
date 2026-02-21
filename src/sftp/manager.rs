//! SFTP文件管理器
//! 处理SFTP文件操作

use anyhow::Result;
use ssh2::Session;
use std::path::Path;

/// 文件信息
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub is_directory: bool,
    pub permissions: String,
    pub modified: String,
}

/// SFTP管理器
pub struct SftpManager {
    session: Session,
}

impl SftpManager {
    /// 创建SFTP管理器
    pub fn new(session: Session) -> Result<Self> {
        Ok(Self { session })
    }

    /// 列出目录内容
    pub fn list_directory(&self, path: &str) -> Result<Vec<FileInfo>> {
        let sftp = self.session.sftp()?;
        let entries = sftp.readdir(Path::new(path))?;

        let mut files = Vec::new();
        for (entry_path, stat) in entries {
            let name = entry_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let info = FileInfo {
                name,
                size: stat.size.unwrap_or(0),
                is_directory: stat.is_dir(),
                permissions: format!("{:o}", stat.perm.unwrap_or(0)),
                modified: format!("{:?}", stat.mtime.unwrap_or(0)),
            };
            files.push(info);
        }

        Ok(files)
    }

    /// 创建目录
    pub fn create_directory(&self, path: &str) -> Result<()> {
        let sftp = self.session.sftp()?;
        sftp.mkdir(Path::new(path), 0o755)?;
        Ok(())
    }

    /// 删除文件或目录
    pub fn remove_file(&self, path: &str) -> Result<()> {
        let sftp = self.session.sftp()?;
        let stat = sftp.stat(Path::new(path))?;

        if stat.is_dir() {
            sftp.rmdir(Path::new(path))?;
        } else {
            sftp.unlink(Path::new(path))?;
        }

        Ok(())
    }

    /// 重命名文件
    pub fn rename_file(&self, old_path: &str, new_path: &str) -> Result<()> {
        let sftp = self.session.sftp()?;
        sftp.rename(Path::new(old_path), Path::new(new_path), None)?;
        Ok(())
    }

    /// 上传文件
    pub fn upload_file(&self, local_path: &str, remote_path: &str) -> Result<()> {
        let sftp = self.session.sftp()?;
        let mut remote_file = sftp.create(Path::new(remote_path))?;
        let local_file = std::fs::File::open(local_path)?;
        let mut reader = std::io::BufReader::new(local_file);

        std::io::copy(&mut reader, &mut remote_file)?;
        Ok(())
    }

    /// 下载文件
    pub fn download_file(&self, remote_path: &str, local_path: &str) -> Result<()> {
        let sftp = self.session.sftp()?;
        let remote_file = sftp.open(Path::new(remote_path))?;
        let mut local_file = std::fs::File::create(local_path)?;
        let mut reader = std::io::BufReader::new(remote_file);

        std::io::copy(&mut reader, &mut local_file)?;
        Ok(())
    }
}
