//! 国际化支持模块
//! 提供多语言支持功能

use std::collections::HashMap;

/// 语言枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Chinese,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Chinese
    }
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s {
            "en" | "en-US" | "English" => Language::English,
            _ => Language::Chinese,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Language::Chinese => "zh-CN",
            Language::English => "en",
        }
    }
}

/// 国际化字符串键
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum I18nKey {
    // 菜单栏
    MenuFile,
    MenuEdit,
    MenuHelp,
    MenuNewConnection,
    MenuExit,
    MenuSettings,
    MenuAbout,
    MenuImportConfig,
    MenuExportConfig,
    MenuOverwrite,
    MenuMerge,
    SelectConfigFile,
    ExportPath,
    ImportMode,
    ImportButton,
    ExportButton,
    SelectPreset,
    BrightColors,

    // 连接管理
    ConnectionManagement,
    NewConnection,
    NewGroup,
    Recent,
    Groups,
    NoHistory,
    NoGroups,
    EmptyGroup,

    // 连接对话框
    ConnectionName,
    HostAddress,
    Port,
    Username,
    AuthMethod,
    Password,
    PrivateKey,
    PrivateKeyPath,
    SaveToHistory,
    Group,
    QuickConnect,
    TestConnection,
    SaveToGroup,
    Connect,
    Cancel,
    Browse,
    Edit,
    EditConnection,
    RemoveFromRecent,
    RemoveFromGroup,
    DeletePermanently,

    // 分组对话框
    GroupName,
    GroupDescription,
    CreateGroup,
    EditGroup,
    DeleteGroup,
    Save,
    Create,

    // 状态信息
    Connecting,
    Connected,
    Disconnected,
    ConnectionError,
    NoActiveConnections,

    // 操作提示
    ConnectionRemoved,
    ConnectionDeleted,
    GroupDeleted,
    ConnectionSaved,

    // 关于对话框
    AboutTitle,
    SoftwareName,
    VersionInfo,
    Version,
    CommitId,
    GitBranch,
    BuildTime,
    BuildUser,
    Description,
    SoftwareDescription,
    KeyFeatures,
    FeatureSSH,
    FeatureSFTP,
    FeatureTerminal,
    FeatureConnection,
    FeatureLanguage,
    Copyright,

    // 设置对话框
    SettingsTitle,
    Language,
    Theme,
    TerminalSettings,
    Appearance,
    FontSize,
    FontFamily,
    CursorBlink,
    DarkTheme,
    LightTheme,
    Chinese,
    English,

    // 主题设置
    AutoTheme,
    CurrentSystemTheme,
    DarkThemeName,
    LightThemeName,
    Unknown,

    // 自定义主题
    CustomTheme,
    ForegroundColor,
    BackgroundColor,
    CursorColor,
    SelectionColor,
    StandardColors,
    ResetToDefault,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    // 分组功能
    NoGroup,
    SelectGroup,
    AddConnection,

    // 连接表单
    PasswordLabel,

    // 通用
    Ok,
    Close,
    Yes,
    No,
    // 欢迎信息
    WelcomeMessage,
    // 历史记录操作
    ClearAllHistory,
    // 连接信息显示
    ConnectedTo,
    Host,
    Status,
    // 测试连接结果
    TestConnectionSuccess,
    TestConnectionFailed,
    TestConnectionLatency,
}

/// 国际化管理器
pub struct I18nManager {
    current_language: Language,
    translations: HashMap<Language, HashMap<I18nKey, &'static str>>,
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}

impl I18nManager {
    pub fn new() -> Self {
        let mut manager = Self {
            current_language: Language::Chinese,
            translations: HashMap::new(),
        };

        manager.init_translations();
        manager
    }

    /// 初始化翻译内容
    fn init_translations(&mut self) {
        // 中文翻译
        let mut zh_translations = HashMap::new();
        zh_translations.insert(I18nKey::MenuFile, "文件");
        zh_translations.insert(I18nKey::MenuEdit, "编辑");
        zh_translations.insert(I18nKey::MenuHelp, "帮助");
        zh_translations.insert(I18nKey::MenuNewConnection, "新建连接");
        zh_translations.insert(I18nKey::MenuExit, "退出");
        zh_translations.insert(I18nKey::MenuSettings, "设置");
        zh_translations.insert(I18nKey::MenuAbout, "关于");
        zh_translations.insert(I18nKey::MenuImportConfig, "导入配置");
        zh_translations.insert(I18nKey::MenuExportConfig, "导出配置");
        zh_translations.insert(I18nKey::MenuOverwrite, "覆盖当前配置");
        zh_translations.insert(I18nKey::MenuMerge, "合并到当前配置");
        zh_translations.insert(I18nKey::SelectConfigFile, "选择配置文件:");
        zh_translations.insert(I18nKey::ExportPath, "导出路径:");
        zh_translations.insert(I18nKey::ImportMode, "导入模式:");
        zh_translations.insert(I18nKey::ImportButton, "导入");
        zh_translations.insert(I18nKey::ExportButton, "导出");
        zh_translations.insert(I18nKey::SelectPreset, "选择配色方案预设");
        zh_translations.insert(I18nKey::BrightColors, "明亮色");

        zh_translations.insert(I18nKey::ConnectionManagement, "连接管理");
        zh_translations.insert(I18nKey::NewConnection, "新建连接");
        zh_translations.insert(I18nKey::NewGroup, "新建分组");
        zh_translations.insert(I18nKey::Recent, "最近");
        zh_translations.insert(I18nKey::Groups, "分组");
        zh_translations.insert(I18nKey::NoHistory, "暂无历史记录");
        zh_translations.insert(I18nKey::NoGroups, "暂无分组");
        zh_translations.insert(I18nKey::EmptyGroup, "空分组");

        zh_translations.insert(I18nKey::ConnectionName, "连接名称:");
        zh_translations.insert(I18nKey::HostAddress, "主机地址:");
        zh_translations.insert(I18nKey::Port, "端口:");
        zh_translations.insert(I18nKey::Username, "用户名:");
        zh_translations.insert(I18nKey::AuthMethod, "认证方式:");
        zh_translations.insert(I18nKey::Password, "密码:");
        zh_translations.insert(I18nKey::PrivateKey, "密钥文件");
        zh_translations.insert(I18nKey::PrivateKeyPath, "私钥路径:");
        zh_translations.insert(I18nKey::SaveToHistory, "保存到连接历史");
        zh_translations.insert(I18nKey::Group, "分组:");
        zh_translations.insert(I18nKey::QuickConnect, "快速连接");
        zh_translations.insert(I18nKey::TestConnection, "测试连接");
        zh_translations.insert(I18nKey::SaveToGroup, "保存到分组");
        zh_translations.insert(I18nKey::Connect, "连接");
        zh_translations.insert(I18nKey::Cancel, "取消");
        zh_translations.insert(I18nKey::Browse, "浏览");
        zh_translations.insert(I18nKey::Edit, "编辑");
        zh_translations.insert(I18nKey::EditConnection, "编辑连接");
        zh_translations.insert(I18nKey::RemoveFromRecent, "从最近列表中移除");
        zh_translations.insert(I18nKey::RemoveFromGroup, "从分组中移除");
        zh_translations.insert(I18nKey::DeletePermanently, "彻底从磁盘删除");

        zh_translations.insert(I18nKey::GroupName, "分组名称:");
        zh_translations.insert(I18nKey::GroupDescription, "分组描述:");
        zh_translations.insert(I18nKey::CreateGroup, "创建分组");
        zh_translations.insert(I18nKey::EditGroup, "编辑分组");
        zh_translations.insert(I18nKey::DeleteGroup, "删除分组");
        zh_translations.insert(I18nKey::Save, "保存");
        zh_translations.insert(I18nKey::Create, "创建");

        zh_translations.insert(I18nKey::Connecting, "连接中");
        zh_translations.insert(I18nKey::Connected, "已连接");
        zh_translations.insert(I18nKey::Disconnected, "已断开");
        zh_translations.insert(I18nKey::ConnectionError, "连接错误");
        zh_translations.insert(I18nKey::NoActiveConnections, "暂无活动连接");

        zh_translations.insert(I18nKey::ConnectionRemoved, "连接已移除");
        zh_translations.insert(I18nKey::ConnectionDeleted, "连接已删除");
        zh_translations.insert(I18nKey::GroupDeleted, "分组已删除");
        zh_translations.insert(I18nKey::ConnectionSaved, "连接已保存");

        zh_translations.insert(I18nKey::AboutTitle, "关于 TermLink");
        zh_translations.insert(I18nKey::SoftwareName, "软件名称");
        zh_translations.insert(I18nKey::VersionInfo, "版本信息");
        zh_translations.insert(I18nKey::Version, "版本");
        zh_translations.insert(I18nKey::CommitId, "提交ID");
        zh_translations.insert(I18nKey::GitBranch, "Git分支");
        zh_translations.insert(I18nKey::BuildTime, "构建时间");
        zh_translations.insert(I18nKey::BuildUser, "构建用户");
        zh_translations.insert(I18nKey::Description, "描述");
        zh_translations.insert(I18nKey::SoftwareDescription, "TermLink是一个现代化的终端连接和文件传输工具，支持SSH连接、SFTP文件传输、终端仿真等功能。");
        zh_translations.insert(I18nKey::KeyFeatures, "主要特性：");
        zh_translations.insert(I18nKey::FeatureSSH, "• SSH安全连接");
        zh_translations.insert(I18nKey::FeatureSFTP, "• SFTP文件传输");
        zh_translations.insert(I18nKey::FeatureTerminal, "• 终端仿真");
        zh_translations.insert(I18nKey::FeatureConnection, "• 连接管理");
        zh_translations.insert(I18nKey::FeatureLanguage, "• 多语言支持");
        zh_translations.insert(I18nKey::Copyright, "© 2024 TermLink 开发团队");

        // 设置对话框
        zh_translations.insert(I18nKey::SettingsTitle, "设置");
        zh_translations.insert(I18nKey::Language, "语言");
        zh_translations.insert(I18nKey::Theme, "主题");
        zh_translations.insert(I18nKey::TerminalSettings, "终端设置");
        zh_translations.insert(I18nKey::Appearance, "外观");
        zh_translations.insert(I18nKey::FontSize, "字体大小");
        zh_translations.insert(I18nKey::FontFamily, "字体");
        zh_translations.insert(I18nKey::CursorBlink, "光标闪烁");
        zh_translations.insert(I18nKey::DarkTheme, "深色主题");
        zh_translations.insert(I18nKey::LightTheme, "浅色主题");
        zh_translations.insert(I18nKey::Chinese, "中文");
        zh_translations.insert(I18nKey::English, "English");

        // 主题设置
        zh_translations.insert(I18nKey::AutoTheme, "自动");
        zh_translations.insert(I18nKey::CurrentSystemTheme, "当前系统主题:");
        zh_translations.insert(I18nKey::DarkThemeName, "深色");
        zh_translations.insert(I18nKey::LightThemeName, "浅色");
        zh_translations.insert(I18nKey::Unknown, "未知");

        // 自定义主题
        zh_translations.insert(I18nKey::CustomTheme, "自定义主题");
        zh_translations.insert(I18nKey::ForegroundColor, "前景色");
        zh_translations.insert(I18nKey::BackgroundColor, "背景色");
        zh_translations.insert(I18nKey::CursorColor, "光标颜色");
        zh_translations.insert(I18nKey::SelectionColor, "选择背景色");
        zh_translations.insert(I18nKey::StandardColors, "标准颜色");
        zh_translations.insert(I18nKey::ResetToDefault, "重置为默认");
        zh_translations.insert(I18nKey::Black, "黑色");
        zh_translations.insert(I18nKey::Red, "红色");
        zh_translations.insert(I18nKey::Green, "绿色");
        zh_translations.insert(I18nKey::Yellow, "黄色");
        zh_translations.insert(I18nKey::Blue, "蓝色");
        zh_translations.insert(I18nKey::Magenta, "品红色");
        zh_translations.insert(I18nKey::Cyan, "青色");
        zh_translations.insert(I18nKey::White, "白色");

        // 分组功能
        zh_translations.insert(I18nKey::NoGroup, "未分组");
        zh_translations.insert(I18nKey::SelectGroup, "选择分组");
        zh_translations.insert(I18nKey::AddConnection, "添加连接");

        // 连接表单
        zh_translations.insert(I18nKey::PasswordLabel, "密码:");

        // 欢迎信息
        zh_translations.insert(I18nKey::WelcomeMessage, "欢迎使用 TermLink");

        // 历史记录操作
        zh_translations.insert(I18nKey::ClearAllHistory, "🗑 清空所有历史");

        // 连接信息显示
        zh_translations.insert(I18nKey::ConnectedTo, "连接到");
        zh_translations.insert(I18nKey::Host, "主机");
        zh_translations.insert(I18nKey::Status, "状态");

        // 测试连接结果
        zh_translations.insert(I18nKey::TestConnectionSuccess, "连接测试成功");
        zh_translations.insert(I18nKey::TestConnectionFailed, "连接测试失败");
        zh_translations.insert(I18nKey::TestConnectionLatency, "延迟");

        zh_translations.insert(I18nKey::Ok, "确定");
        zh_translations.insert(I18nKey::Close, "关闭");
        zh_translations.insert(I18nKey::Yes, "是");
        zh_translations.insert(I18nKey::No, "否");

        self.translations.insert(Language::Chinese, zh_translations);

        // 英文翻译
        let mut en_translations = HashMap::new();
        en_translations.insert(I18nKey::MenuFile, "File");
        en_translations.insert(I18nKey::MenuEdit, "Edit");
        en_translations.insert(I18nKey::MenuHelp, "Help");
        en_translations.insert(I18nKey::MenuNewConnection, "New Connection");
        en_translations.insert(I18nKey::MenuExit, "Exit");
        en_translations.insert(I18nKey::MenuSettings, "Settings");
        en_translations.insert(I18nKey::MenuAbout, "About");
        en_translations.insert(I18nKey::MenuImportConfig, "Import Config");
        en_translations.insert(I18nKey::MenuExportConfig, "Export Config");
        en_translations.insert(I18nKey::MenuOverwrite, "Overwrite Current Config");
        en_translations.insert(I18nKey::MenuMerge, "Merge into Current Config");
        en_translations.insert(I18nKey::SelectConfigFile, "Select Config File:");
        en_translations.insert(I18nKey::ExportPath, "Export Path:");
        en_translations.insert(I18nKey::ImportMode, "Import Mode:");
        en_translations.insert(I18nKey::ImportButton, "Import");
        en_translations.insert(I18nKey::ExportButton, "Export");
        en_translations.insert(I18nKey::SelectPreset, "Select Color Preset");
        en_translations.insert(I18nKey::BrightColors, "Bright Colors");

        en_translations.insert(I18nKey::ConnectionManagement, "Connection Management");
        en_translations.insert(I18nKey::NewConnection, "New Connection");
        en_translations.insert(I18nKey::NewGroup, "New Group");
        en_translations.insert(I18nKey::Recent, "Recent");
        en_translations.insert(I18nKey::Groups, "Groups");
        en_translations.insert(I18nKey::NoHistory, "No history records");
        en_translations.insert(I18nKey::NoGroups, "No groups");
        en_translations.insert(I18nKey::EmptyGroup, "Empty group");

        en_translations.insert(I18nKey::ConnectionName, "Connection Name:");
        en_translations.insert(I18nKey::HostAddress, "Host Address:");
        en_translations.insert(I18nKey::Port, "Port:");
        en_translations.insert(I18nKey::Username, "Username:");
        en_translations.insert(I18nKey::AuthMethod, "Authentication:");
        en_translations.insert(I18nKey::Password, "Password:");
        en_translations.insert(I18nKey::PrivateKey, "Key File");
        en_translations.insert(I18nKey::PrivateKeyPath, "Private Key Path:");
        en_translations.insert(I18nKey::SaveToHistory, "Save to Connection History");
        en_translations.insert(I18nKey::Group, "Group:");
        en_translations.insert(I18nKey::QuickConnect, "Quick Connect");
        en_translations.insert(I18nKey::TestConnection, "Test Connection");
        en_translations.insert(I18nKey::SaveToGroup, "Save to Group");
        en_translations.insert(I18nKey::Connect, "Connect");
        en_translations.insert(I18nKey::Cancel, "Cancel");
        en_translations.insert(I18nKey::Browse, "Browse");
        en_translations.insert(I18nKey::Edit, "Edit");
        en_translations.insert(I18nKey::EditConnection, "Edit Connection");
        en_translations.insert(I18nKey::RemoveFromRecent, "Remove from Recent");
        en_translations.insert(I18nKey::RemoveFromGroup, "Remove from Group");
        en_translations.insert(I18nKey::DeletePermanently, "Delete Permanently");

        en_translations.insert(I18nKey::GroupName, "Group Name:");
        en_translations.insert(I18nKey::GroupDescription, "Group Description:");
        en_translations.insert(I18nKey::CreateGroup, "Create Group");
        en_translations.insert(I18nKey::EditGroup, "Edit Group");
        en_translations.insert(I18nKey::DeleteGroup, "Delete Group");
        en_translations.insert(I18nKey::Save, "Save");
        en_translations.insert(I18nKey::Create, "Create");

        en_translations.insert(I18nKey::Connecting, "Connecting");
        en_translations.insert(I18nKey::Connected, "Connected");
        en_translations.insert(I18nKey::Disconnected, "Disconnected");
        en_translations.insert(I18nKey::ConnectionError, "Connection Error");
        en_translations.insert(I18nKey::NoActiveConnections, "No active connections");

        en_translations.insert(I18nKey::ConnectionRemoved, "Connection removed");
        en_translations.insert(I18nKey::ConnectionDeleted, "Connection deleted");
        en_translations.insert(I18nKey::GroupDeleted, "Group deleted");
        en_translations.insert(I18nKey::ConnectionSaved, "Connection saved");

        en_translations.insert(I18nKey::AboutTitle, "About TermLink");
        en_translations.insert(I18nKey::SoftwareName, "Software Name");
        en_translations.insert(I18nKey::Version, "Version");
        en_translations.insert(I18nKey::VersionInfo, "Version Info");
        en_translations.insert(I18nKey::CommitId, "Commit ID");
        en_translations.insert(I18nKey::GitBranch, "Git Branch");
        en_translations.insert(I18nKey::BuildTime, "Build Time");
        en_translations.insert(I18nKey::BuildUser, "Build User");
        en_translations.insert(I18nKey::Description, "Description");
        en_translations.insert(I18nKey::SoftwareDescription, "TermLink is a modern terminal connection and file transfer tool that supports SSH connections, SFTP file transfer, and terminal emulation.");
        en_translations.insert(I18nKey::KeyFeatures, "Key Features:");
        en_translations.insert(I18nKey::FeatureSSH, "• Secure SSH Connection");
        en_translations.insert(I18nKey::FeatureSFTP, "• SFTP File Transfer");
        en_translations.insert(I18nKey::FeatureTerminal, "• Terminal Emulation");
        en_translations.insert(I18nKey::FeatureConnection, "• Connection Management");
        en_translations.insert(I18nKey::FeatureLanguage, "• Multi-language Support");
        en_translations.insert(I18nKey::Copyright, "© 2024 TermLink Development Team");

        // Settings dialog
        en_translations.insert(I18nKey::SettingsTitle, "Settings");
        en_translations.insert(I18nKey::Language, "Language");
        en_translations.insert(I18nKey::Theme, "Theme");
        en_translations.insert(I18nKey::TerminalSettings, "Terminal Settings");
        en_translations.insert(I18nKey::Appearance, "Appearance");
        en_translations.insert(I18nKey::FontSize, "Font Size");
        en_translations.insert(I18nKey::FontFamily, "Font Family");
        en_translations.insert(I18nKey::CursorBlink, "Cursor Blink");
        en_translations.insert(I18nKey::DarkTheme, "Dark Theme");
        en_translations.insert(I18nKey::LightTheme, "Light Theme");
        en_translations.insert(I18nKey::Chinese, "中文");
        en_translations.insert(I18nKey::English, "English");

        // Theme settings
        en_translations.insert(I18nKey::AutoTheme, "Auto");
        en_translations.insert(I18nKey::CurrentSystemTheme, "Current System Theme:");
        en_translations.insert(I18nKey::DarkThemeName, "Dark");
        en_translations.insert(I18nKey::LightThemeName, "Light");
        en_translations.insert(I18nKey::Unknown, "Unknown");

        // Custom theme
        en_translations.insert(I18nKey::CustomTheme, "Custom Theme");
        en_translations.insert(I18nKey::ForegroundColor, "Foreground Color");
        en_translations.insert(I18nKey::BackgroundColor, "Background Color");
        en_translations.insert(I18nKey::CursorColor, "Cursor Color");
        en_translations.insert(I18nKey::SelectionColor, "Selection Background");
        en_translations.insert(I18nKey::StandardColors, "Standard Colors");
        en_translations.insert(I18nKey::ResetToDefault, "Reset to Default");
        en_translations.insert(I18nKey::Black, "Black");
        en_translations.insert(I18nKey::Red, "Red");
        en_translations.insert(I18nKey::Green, "Green");
        en_translations.insert(I18nKey::Yellow, "Yellow");
        en_translations.insert(I18nKey::Blue, "Blue");
        en_translations.insert(I18nKey::Magenta, "Magenta");
        en_translations.insert(I18nKey::Cyan, "Cyan");
        en_translations.insert(I18nKey::White, "White");

        // Group functionality
        en_translations.insert(I18nKey::NoGroup, "No Group");
        en_translations.insert(I18nKey::SelectGroup, "Select Group");
        en_translations.insert(I18nKey::AddConnection, "Add Connection");

        // Connection form
        en_translations.insert(I18nKey::PasswordLabel, "Password:");

        // Welcome message
        en_translations.insert(I18nKey::WelcomeMessage, "Welcome to TermLink");

        // History operations
        en_translations.insert(I18nKey::ClearAllHistory, "🗑 Clear All History");

        // Connection info display
        en_translations.insert(I18nKey::ConnectedTo, "Connected to");
        en_translations.insert(I18nKey::Host, "Host");
        en_translations.insert(I18nKey::Status, "Status");

        // Test connection results
        en_translations.insert(I18nKey::TestConnectionSuccess, "Connection test successful");
        en_translations.insert(I18nKey::TestConnectionFailed, "Connection test failed");
        en_translations.insert(I18nKey::TestConnectionLatency, "Latency");

        en_translations.insert(I18nKey::Ok, "OK");
        en_translations.insert(I18nKey::Close, "Close");
        en_translations.insert(I18nKey::Yes, "Yes");
        en_translations.insert(I18nKey::No, "No");

        self.translations.insert(Language::English, en_translations);
    }

    /// 设置当前语言
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    /// 获取当前语言
    pub fn get_language(&self) -> &Language {
        &self.current_language
    }

    /// 获取翻译文本
    pub fn get(&self, key: I18nKey) -> &'static str {
        self.translations
            .get(&self.current_language)
            .and_then(|map| map.get(&key))
            .copied()
            .unwrap_or("Unknown")
    }

    /// 获取所有支持的语言
    pub fn get_supported_languages() -> Vec<Language> {
        vec![Language::Chinese, Language::English]
    }
}
