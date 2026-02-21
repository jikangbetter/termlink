//! UI组件模块
//! 包含主应用的用户界面实现

use crate::config::settings::ConnectionGroup;
use crate::config::settings::ThemeMode;
use crate::config::{AppSettings, ConnectionConfig};
use crate::i18n::{I18nKey, I18nManager, Language};
use crate::ssh::{ConnectionManager, ConnectionTestResult, SessionState, SshSession};
use crate::terminal::TerminalEmulator;
use eframe::egui;
use std::sync::{Arc, Mutex};

// 获取构建时信息
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn get_commit_hash() -> String {
    std::env!("GIT_COMMIT_ID").to_string()
}

fn get_git_branch() -> String {
    std::env!("GIT_BRANCH").to_string()
}

fn get_build_time() -> String {
    std::env!("BUILD_TIME").to_string()
}

fn get_build_user() -> String {
    std::env!("BUILD_USER").to_string()
}

/// 关于对话框
#[derive(Default)]
pub struct AboutDialog {
    pub show: bool,
}

impl AboutDialog {
    pub fn new() -> Self {
        Self { show: false }
    }

    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn ui(&mut self, ctx: &egui::Context, i18n: &I18nManager) {
        if !self.show {
            return;
        }

        egui::Window::new(i18n.get(I18nKey::AboutTitle))
            .default_width(400.0)
            .default_height(300.0)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // 软件Logo或图标（这里用文本代替）

                    // 软件标题
                    ui.heading("TermLink");
                    ui.label(i18n.get(I18nKey::Description));
                    ui.add_space(15.0);

                    // 版本信息
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading(i18n.get(I18nKey::VersionInfo));
                            ui.separator();

                            ui.horizontal(|ui| {
                                ui.strong(i18n.get(I18nKey::Version));
                                ui.label(get_version());
                            });

                            ui.horizontal(|ui| {
                                ui.strong(i18n.get(I18nKey::CommitId));
                                ui.label(get_commit_hash());
                            });

                            ui.horizontal(|ui| {
                                ui.strong(i18n.get(I18nKey::GitBranch));
                                ui.label(get_git_branch());
                            });

                            ui.horizontal(|ui| {
                                ui.strong(i18n.get(I18nKey::BuildTime));
                                ui.label(get_build_time());
                            });

                            ui.horizontal(|ui| {
                                ui.strong(i18n.get(I18nKey::BuildUser));
                                ui.label(get_build_user());
                            });
                        });
                    });

                    ui.add_space(15.0);

                    // 软件描述
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading(i18n.get(I18nKey::Description));
                            ui.separator();
                            ui.label(i18n.get(I18nKey::SoftwareDescription));
                            ui.add_space(5.0);
                            ui.label(i18n.get(I18nKey::KeyFeatures));
                            ui.horizontal(|ui| {
                                ui.label(i18n.get(I18nKey::FeatureSSH));
                            });
                            ui.horizontal(|ui| {
                                ui.label(i18n.get(I18nKey::FeatureSFTP));
                            });
                            ui.horizontal(|ui| {
                                ui.label(i18n.get(I18nKey::FeatureTerminal));
                            });
                            ui.horizontal(|ui| {
                                ui.label(i18n.get(I18nKey::FeatureConnection));
                            });
                            ui.horizontal(|ui| {
                                ui.label(i18n.get(I18nKey::FeatureLanguage));
                            });
                        });
                    });

                    ui.add_space(10.0);

                    // 确定按钮
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if ui.button(i18n.get(I18nKey::Ok)).clicked() {
                            self.show = false;
                        }
                    });
                });
            });
    }
}

/// 设置对话框
#[derive(Default)]
pub struct SettingsDialog {
    pub show: bool,
    /// 本地设置副本，用于临时修改
    pub temp_settings: AppSettings,
    /// 本地语言设置
    pub temp_language: Language,
    /// 本地主题设置
    pub temp_theme: String,
}

impl SettingsDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self, current_settings: &AppSettings, current_language: &Language) {
        self.temp_settings = current_settings.clone();
        self.temp_language = current_language.clone();
        self.temp_theme = current_settings.get_current_theme();
        self.show = true;
    }

    pub fn ui<F>(&mut self, ctx: &egui::Context, i18n: &I18nManager, on_settings_changed: F)
    where
        F: FnOnce(AppSettings, Language),
    {
        if !self.show {
            return;
        }

        egui::Window::new(i18n.get(I18nKey::SettingsTitle))
            .default_width(500.0)
            .default_height(400.0)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // 语言设置
                    ui.group(|ui| {
                        ui.heading(i18n.get(I18nKey::Language));
                        ui.separator();

                        egui::ComboBox::from_label(i18n.get(I18nKey::Language))
                            .selected_text(match self.temp_language {
                                Language::Chinese => i18n.get(I18nKey::Chinese),
                                Language::English => i18n.get(I18nKey::English),
                            })
                            .show_ui(ui, |ui| {
                                if ui
                                    .selectable_value(
                                        &mut self.temp_language,
                                        Language::Chinese,
                                        i18n.get(I18nKey::Chinese),
                                    )
                                    .clicked()
                                {
                                    // 语言切换时立即更新界面
                                }
                                if ui
                                    .selectable_value(
                                        &mut self.temp_language,
                                        Language::English,
                                        i18n.get(I18nKey::English),
                                    )
                                    .clicked()
                                {
                                    // 语言切换时立即更新界面
                                }
                            });
                    });

                    ui.add_space(10.0);

                    // 外观设置
                    ui.group(|ui| {
                        ui.heading(i18n.get(I18nKey::Appearance));
                        ui.separator();

                        egui::ComboBox::from_label(i18n.get(I18nKey::Theme))
                            .selected_text(if self.temp_theme == "dark" {
                                i18n.get(I18nKey::DarkTheme)
                            } else {
                                i18n.get(I18nKey::LightTheme)
                            })
                            .show_ui(ui, |ui| {
                                if ui
                                    .selectable_value(
                                        &mut self.temp_theme,
                                        "dark".to_string(),
                                        i18n.get(I18nKey::DarkTheme),
                                    )
                                    .clicked()
                                {
                                    self.temp_settings.appearance.theme_mode = ThemeMode::Dark;
                                    self.temp_settings.terminal.theme = "dark".to_string();
                                }
                                if ui
                                    .selectable_value(
                                        &mut self.temp_theme,
                                        "light".to_string(),
                                        i18n.get(I18nKey::LightTheme),
                                    )
                                    .clicked()
                                {
                                    self.temp_settings.appearance.theme_mode = ThemeMode::Light;
                                    self.temp_settings.terminal.theme = "light".to_string();
                                }
                            });
                    });

                    ui.add_space(10.0);

                    // 终端设置
                    ui.group(|ui| {
                        ui.heading(i18n.get(I18nKey::TerminalSettings));
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label(i18n.get(I18nKey::FontSize));
                            ui.add(
                                egui::DragValue::new(&mut self.temp_settings.terminal.font_size)
                                    .speed(1.0)
                                    .clamp_range(8.0..=32.0),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label(i18n.get(I18nKey::FontFamily));
                            ui.text_edit_singleline(&mut self.temp_settings.terminal.font_family);
                        });

                        ui.checkbox(
                            &mut self.temp_settings.terminal.cursor_blink,
                            i18n.get(I18nKey::CursorBlink),
                        );
                    });

                    ui.add_space(20.0);
                    ui.separator();

                    // 按钮区域
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            if ui.button(i18n.get(I18nKey::Cancel)).clicked() {
                                self.show = false;
                            }

                            if ui.button(i18n.get(I18nKey::Save)).clicked() {
                                // 应用设置
                                self.temp_settings.appearance.language =
                                    self.temp_language.to_str().to_string();
                                // 主题由theme_mode控制，不需要单独设置

                                // 调用回调函数通知设置变更
                                on_settings_changed(
                                    self.temp_settings.clone(),
                                    self.temp_language.clone(),
                                );

                                self.show = false;
                            }
                        });
                    });
                });
            });
    }
}

/// 主应用结构体
pub struct App {
    /// 应用设置
    pub settings: AppSettings,
    /// 国际化管理器
    pub i18n: I18nManager,
    /// 连接管理器
    pub connection_manager: Arc<Mutex<ConnectionManager>>,
    /// 当前选中的会话名称
    pub current_session: Option<String>,
    /// 连接配置表单数据
    pub connection_form: ConnectionForm,
    /// 是否显示连接对话框
    pub show_connection_dialog: bool,
    /// 正在编辑的连接原名（用于更新）
    pub editing_connection_name: Option<String>,
    /// 连接历史记录
    pub connection_history: Vec<ConnectionConfig>,
    /// 连接分组
    pub connection_groups: Vec<ConnectionGroup>,
    /// 是否显示创建分组对话框
    pub show_create_group_dialog: bool,
    /// 正在编辑的分组索引
    pub editing_group_index: Option<usize>,
    /// 是否显示分组视图
    pub show_group_view: bool,
    /// 分组配置表单
    pub group_form: GroupForm,
    /// 连接测试结果
    pub test_result: Option<ConnectionTestResult>,
    /// 为每个会话维护的终端仿真器
    pub terminal_emulators: std::collections::HashMap<String, TerminalEmulator>,
    /// 上次读取时间
    pub last_read_time: Option<std::time::Instant>,
    /// 关于对话框
    pub about_dialog: AboutDialog,
    /// 设置对话框
    pub settings_dialog: SettingsDialog,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            settings: AppSettings::default(),
            i18n: I18nManager::new(),
            connection_manager: Arc::new(Mutex::new(ConnectionManager::new())),
            current_session: None,
            connection_form: ConnectionForm::default(),
            show_connection_dialog: false,
            editing_connection_name: None,
            connection_history: Vec::new(),
            connection_groups: Vec::new(),
            show_create_group_dialog: false,
            editing_group_index: None,
            show_group_view: false,
            group_form: GroupForm::default(),
            test_result: None,
            terminal_emulators: std::collections::HashMap::new(),
            last_read_time: None,
            about_dialog: AboutDialog::new(),
            settings_dialog: SettingsDialog::new(),
        };

        // 加载保存的应用状态
        app.load_app_state();

        // 应用保存的语言设置
        match app.settings.appearance.language.as_str() {
            "en" => app.i18n.set_language(Language::English),
            _ => app.i18n.set_language(Language::Chinese),
        }

        // 检测系统主题并更新设置
        let system_theme = crate::utils::helpers::detect_system_theme();
        app.settings.appearance.system_theme = system_theme;

        // 应用当前主题设置
        app.settings.terminal.theme = app.settings.get_current_theme();
        app
    }
}

/// 连接配置表单
#[derive(Default)]
pub struct ConnectionForm {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_key_auth: bool,
    pub private_key_path: String,
    pub save_to_history: bool,
    pub group: Option<String>,
}

/// 分组配置表单
#[derive(Default)]
pub struct GroupForm {
    pub name: String,
    pub description: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 如果终端有焦点，禁用 egui 的默认复制/粘贴/剪切快捷键
        // 这样 Ctrl+C/X/V 才能传递到终端
        if self.current_session.is_some() {
            ctx.input_mut(|i| {
                // 消费掉这些快捷键，防止 egui 默认处理
                i.consume_key(egui::Modifiers::CTRL, egui::Key::C);
                i.consume_key(egui::Modifiers::CTRL, egui::Key::X);
                i.consume_key(egui::Modifiers::CTRL, egui::Key::V);
                i.consume_key(egui::Modifiers::CTRL, egui::Key::A);
            });
        }

        // 应用主题色，确保手动切换主题能立即生效
        let current_theme = self.settings.get_current_theme();
        let is_currently_dark = ctx.style().visuals.dark_mode;
        let should_be_dark = current_theme == "dark";

        if is_currently_dark != should_be_dark {
            if should_be_dark {
                ctx.set_visuals(egui::Visuals::dark());
            } else {
                ctx.set_visuals(egui::Visuals::light());
            }
        }

        // 添加调试信息
        // println!("Update called at {:?}", std::time::Instant::now());

        // 读取终端数据（关键：这必须在UI构建之前执行）
        self.read_from_terminal();

        // 主窗口布局
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu_bar(ui);
        });

        egui::SidePanel::left("connections_panel")
            .resizable(true)
            .default_width(240.0)
            .show(ctx, |ui| {
                self.connections_panel(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_content(ui);
        });

        // 模态对话框
        if self.show_connection_dialog {
            self.connection_dialog(ctx);
        }

        if self.show_create_group_dialog {
            self.create_group_dialog(ctx);
        }

        // 渲染对话框
        self.about_dialog.ui(ctx, &self.i18n);

        // 渲染设置对话框
        if self.settings_dialog.show {
            egui::Window::new(self.i18n.get(I18nKey::SettingsTitle))
                .default_width(600.0)
                .default_height(500.0)
                .collapsible(false)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        // 语言设置
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.heading(self.i18n.get(I18nKey::Language));
                                ui.separator();

                                ui.horizontal(|ui| {
                                    ui.label(self.i18n.get(I18nKey::Language));
                                    egui::ComboBox::from_id_source("language_selector")
                                        .selected_text(
                                            match self.settings.appearance.language.as_str() {
                                                "zh-CN" => self.i18n.get(I18nKey::Chinese),
                                                "en" => self.i18n.get(I18nKey::English),
                                                _ => self.i18n.get(I18nKey::Chinese),
                                            },
                                        )
                                        .show_ui(ui, |ui| {
                                            if ui
                                                .selectable_label(
                                                    self.settings.appearance.language == "zh-CN",
                                                    self.i18n.get(I18nKey::Chinese),
                                                )
                                                .clicked()
                                            {
                                                self.settings.appearance.language =
                                                    "zh-CN".to_string();
                                                self.i18n
                                                    .set_language(crate::i18n::Language::Chinese);
                                            }
                                            if ui
                                                .selectable_label(
                                                    self.settings.appearance.language == "en",
                                                    self.i18n.get(I18nKey::English),
                                                )
                                                .clicked()
                                            {
                                                self.settings.appearance.language =
                                                    "en".to_string();
                                                self.i18n
                                                    .set_language(crate::i18n::Language::English);
                                            }
                                        });
                                });
                            });
                        });

                        ui.add_space(10.0);

                        // 外观设置
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.heading(self.i18n.get(I18nKey::Appearance));
                                ui.separator();

                                ui.horizontal(|ui| {
                                    ui.label(self.i18n.get(I18nKey::Theme));
                                    egui::ComboBox::from_id_source("theme_selector")
                                        .selected_text(
                                            self.settings.get_theme_mode_display(&self.i18n),
                                        )
                                        .show_ui(ui, |ui| {
                                            if ui
                                                .selectable_label(
                                                    matches!(
                                                        self.settings.appearance.theme_mode,
                                                        ThemeMode::Auto
                                                    ),
                                                    self.i18n.get(I18nKey::AutoTheme),
                                                )
                                                .clicked()
                                            {
                                                self.settings.appearance.theme_mode =
                                                    ThemeMode::Auto;
                                                self.settings.terminal.theme =
                                                    self.settings.get_current_theme();
                                            }
                                            if ui
                                                .selectable_label(
                                                    matches!(
                                                        self.settings.appearance.theme_mode,
                                                        ThemeMode::Dark
                                                    ),
                                                    self.i18n.get(I18nKey::DarkTheme),
                                                )
                                                .clicked()
                                            {
                                                self.settings.appearance.theme_mode =
                                                    ThemeMode::Dark;
                                                self.settings.terminal.theme =
                                                    self.settings.get_current_theme();
                                            }
                                            if ui
                                                .selectable_label(
                                                    matches!(
                                                        self.settings.appearance.theme_mode,
                                                        ThemeMode::Light
                                                    ),
                                                    self.i18n.get(I18nKey::LightTheme),
                                                )
                                                .clicked()
                                            {
                                                self.settings.appearance.theme_mode =
                                                    ThemeMode::Light;
                                                self.settings.terminal.theme =
                                                    self.settings.get_current_theme();
                                            }
                                        });
                                });

                                // 显示当前系统主题状态
                                if matches!(self.settings.appearance.theme_mode, ThemeMode::Auto) {
                                    ui.horizontal(|ui| {
                                        ui.label(self.i18n.get(I18nKey::CurrentSystemTheme));
                                        ui.label(
                                            match self.settings.appearance.system_theme.as_str() {
                                                "dark" => self.i18n.get(I18nKey::DarkThemeName),
                                                "light" => self.i18n.get(I18nKey::LightThemeName),
                                                _ => self.i18n.get(I18nKey::Unknown),
                                            },
                                        );
                                    });
                                }
                            });
                        });

                        ui.add_space(10.0);

                        // 终端设置
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.heading(self.i18n.get(I18nKey::TerminalSettings));
                                ui.separator();

                                ui.horizontal(|ui| {
                                    ui.label(self.i18n.get(I18nKey::FontSize));
                                    ui.add(
                                        egui::DragValue::new(&mut self.settings.terminal.font_size)
                                            .speed(1.0)
                                            .clamp_range(8.0..=24.0)
                                            .suffix("px"),
                                    );
                                });

                                ui.horizontal(|ui| {
                                    ui.label(self.i18n.get(I18nKey::FontFamily));
                                    ui.text_edit_singleline(
                                        &mut self.settings.terminal.font_family,
                                    );
                                });

                                ui.checkbox(
                                    &mut self.settings.terminal.cursor_blink,
                                    self.i18n.get(I18nKey::CursorBlink),
                                );
                            });
                        });

                        ui.add_space(20.0);
                        ui.separator();

                        // 按钮区域
                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                if ui.button(self.i18n.get(I18nKey::Cancel)).clicked() {
                                    self.settings_dialog.show = false;
                                }

                                if ui.button(self.i18n.get(I18nKey::Save)).clicked() {
                                    // 保存设置
                                    if let Err(e) = self.settings.save() {
                                        eprintln!("保存设置失败: {}", e);
                                    } else {
                                        println!("设置已应用");
                                        // 更新所有终端的主题
                                        self.update_terminal_themes();
                                    }
                                }

                                if ui.button(self.i18n.get(I18nKey::Ok)).clicked() {
                                    // 保存设置并关闭
                                    if let Err(e) = self.settings.save() {
                                        eprintln!("保存设置失败: {}", e);
                                    } else {
                                        println!("设置已保存");
                                        // 更新所有终端的主题
                                        self.update_terminal_themes();
                                    }
                                    self.settings_dialog.show = false;
                                }
                            });
                        });
                    });
                });
        }

        // 请求下一帧更新，但要控制频率
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
}

impl App {
    /// 菜单栏
    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button(self.i18n.get(I18nKey::MenuFile), |ui| {
                if ui
                    .button(self.i18n.get(I18nKey::MenuNewConnection))
                    .clicked()
                {
                    self.connection_form = ConnectionForm::default();
                    self.editing_connection_name = None;
                    self.show_connection_dialog = true;
                    ui.close_menu();
                }
                if ui.button(self.i18n.get(I18nKey::MenuExit)).clicked() {
                    std::process::exit(0);
                }
            });

            ui.menu_button(self.i18n.get(I18nKey::MenuEdit), |ui| {
                if ui.button(self.i18n.get(I18nKey::MenuSettings)).clicked() {
                    self.settings_dialog
                        .show(&self.settings, self.i18n.get_language());
                    ui.close_menu();
                }
            });

            ui.menu_button(self.i18n.get(I18nKey::MenuHelp), |ui| {
                if ui.button(self.i18n.get(I18nKey::MenuAbout)).clicked() {
                    self.about_dialog.show();
                    ui.close_menu();
                }
            });
        });
    }

    /// 连接列表面板（侧边栏集成管理）
    fn connections_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.heading(self.i18n.get(I18nKey::ConnectionManagement));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button("➕")
                        .on_hover_text(self.i18n.get(I18nKey::NewConnection))
                        .clicked()
                    {
                        self.connection_form = ConnectionForm::default();
                        self.editing_connection_name = None;
                        self.show_connection_dialog = true;
                    }
                    if ui
                        .button("📁")
                        .on_hover_text(self.i18n.get(I18nKey::NewGroup))
                        .clicked()
                    {
                        self.group_form = GroupForm::default();
                        self.editing_group_index = None;
                        self.show_create_group_dialog = true;
                    }
                });
            });

            ui.separator();

            // 视图切换控制
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.show_group_view,
                    false,
                    self.i18n.get(I18nKey::Recent),
                );
                ui.selectable_value(
                    &mut self.show_group_view,
                    true,
                    self.i18n.get(I18nKey::Groups),
                );
            });

            ui.separator();

            // 搜索框（可选，先留个占位）
            // ui.text_edit_singleline(&mut self.search_query);

            egui::ScrollArea::vertical().show(ui, |ui| {
                if !self.show_group_view {
                    self.render_sidebar_history(ui);
                } else {
                    self.render_sidebar_groups(ui);
                }
            });
        });
    }

    /// 侧边栏渲染历史记录
    fn render_sidebar_history(&mut self, ui: &mut egui::Ui) {
        // 只显示有最近连接时间记录的项目
        let mut history_items: Vec<(usize, ConnectionConfig)> = self
            .connection_history
            .iter()
            .enumerate()
            .filter(|(_, c)| c.last_connected.is_some())
            .map(|(i, c)| (i, c.clone()))
            .collect();

        // 按连接时间降序排列
        history_items.sort_by(|(_, a), (_, b)| {
            b.last_connected
                .as_ref()
                .unwrap_or(&String::new())
                .cmp(a.last_connected.as_ref().unwrap_or(&String::new()))
        });

        if history_items.is_empty() {
            ui.weak("暂无历史记录");
            return;
        }

        for (original_index, config) in history_items {
            ui.horizontal(|ui| {
                let response = ui.selectable_label(
                    self.current_session
                        .as_ref()
                        .map(|s| self.extract_base_connection_name(s))
                        == Some(config.name.clone()),
                    &config.name,
                );

                if response.clicked() {
                    self.connect_from_history(config.clone());
                }

                response.context_menu(|ui| {
                    if ui
                        .button(&format!("🔄 {}", self.i18n.get(I18nKey::Connect)))
                        .clicked()
                    {
                        self.connect_from_history(config.clone());
                        ui.close_menu();
                    }
                    if ui
                        .button(&format!("✏️ {}", self.i18n.get(I18nKey::Edit)))
                        .clicked()
                    {
                        self.edit_connection(config.clone());
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui
                        .button(&format!("🧹 {}", self.i18n.get(I18nKey::RemoveFromRecent)))
                        .clicked()
                    {
                        self.clear_connection_history(original_index);
                        ui.close_menu();
                    }
                    if ui
                        .button(&format!("🗑️ {}", self.i18n.get(I18nKey::DeletePermanently)))
                        .clicked()
                    {
                        self.delete_connection(original_index);
                        ui.close_menu();
                    }
                });
            });
        }

        ui.separator();
        if ui.button("🧹 清空所有历史").clicked() {
            for config in self.connection_history.iter_mut() {
                config.last_connected = None;
            }
            self.auto_save_state();
        }
    }

    /// 侧边栏渲染分组
    fn render_sidebar_groups(&mut self, ui: &mut egui::Ui) {
        if self.connection_groups.is_empty() {
            ui.weak("暂无分组");
            return;
        }

        // 查找属于该项目的配置在 history 中的原始索引
        let find_history_index = |history: &Vec<ConnectionConfig>, name: &str| {
            history.iter().position(|c| c.name == name)
        };

        let groups_clone = self.connection_groups.clone();
        for (group_index, group) in groups_clone.iter().enumerate() {
            egui::CollapsingHeader::new(&group.name)
                .default_open(false)
                .show(ui, |ui| {
                    for base_conn_name in &group.connections {
                        ui.horizontal(|ui| {
                            let response = ui.selectable_label(false, base_conn_name);
                            // 点击不再直接连接，仅供选择（或通过右键连接）

                            response.context_menu(|ui| {
                                if ui
                                    .button(&format!("🔄 {}", self.i18n.get(I18nKey::Connect)))
                                    .clicked()
                                {
                                    self.connect_from_group(group_index, base_conn_name);
                                    ui.close_menu();
                                }
                                if ui
                                    .button(&format!("✏️ {}", self.i18n.get(I18nKey::Edit)))
                                    .clicked()
                                {
                                    self.edit_connection_from_group(group_index, base_conn_name);
                                    ui.close_menu();
                                }
                                ui.separator();
                                if ui
                                    .button(&format!(
                                        "🗑️ {}",
                                        self.i18n.get(I18nKey::RemoveFromGroup)
                                    ))
                                    .clicked()
                                {
                                    self.remove_connection_from_group(group_index, base_conn_name);
                                    ui.close_menu();
                                }
                                if let Some(h_idx) =
                                    find_history_index(&self.connection_history, base_conn_name)
                                {
                                    if ui
                                        .button(&format!(
                                            "🔥 {}",
                                            self.i18n.get(I18nKey::DeletePermanently)
                                        ))
                                        .clicked()
                                    {
                                        self.delete_connection(h_idx);
                                        ui.close_menu();
                                    }
                                }
                            });
                        });
                    }
                    if group.connections.is_empty() {
                        ui.weak("空分组");
                    }
                })
                .header_response
                .context_menu(|ui| {
                    if ui
                        .button(&format!("✏️ {}", self.i18n.get(I18nKey::EditGroup)))
                        .clicked()
                    {
                        self.edit_group(group_index);
                        ui.close_menu();
                    }
                    if ui
                        .button(&format!("🗑️ {}", self.i18n.get(I18nKey::DeleteGroup)))
                        .clicked()
                    {
                        self.delete_group(group_index);
                        ui.close_menu();
                    }
                });
        }
    }

    /// 关闭会话
    fn close_session(&mut self, session_name: &str) {
        let manager = self.connection_manager.lock().unwrap();
        manager.disconnect(session_name);
        drop(manager);

        // 如果关闭的是当前会话，清除当前会话
        if self.current_session.as_ref().map(|s| s.as_str()) == Some(session_name) {
            self.current_session = None;
        }
    }

    /// 主内容区域
    fn main_content(&mut self, ui: &mut egui::Ui) {
        // 确保当前会话有对应的终端仿真器
        if let Some(ref session_name) = self.current_session {
            if !self.terminal_emulators.contains_key(session_name) {
                let mut emulator = TerminalEmulator::new(24, 80);

                // 设置终端事件回调
                let session_name_clone = session_name.clone();
                let manager_clone = self.connection_manager.clone();
                emulator.set_event_callback(move |event| {
                    if let crate::terminal::TerminalEvent::Resize { rows, cols } = event {
                        let manager = manager_clone.lock().unwrap();
                        if let Some(session) = manager.get_session(&session_name_clone) {
                            if let Err(e) = session.resize_terminal(rows as u32, cols as u32) {
                                eprintln!("调整终端大小失败: {}", e);
                            }
                        }
                    }
                });

                self.terminal_emulators
                    .insert(session_name.clone(), emulator);
            }
        }

        // 显示活动会话标签页
        self.render_session_tabs(ui);

        let mut session_display_info = None;

        if let Some(ref session_name) = self.current_session {
            let manager = self.connection_manager.lock().unwrap();
            if let Some(session_info) = manager.get_session_info(session_name) {
                session_display_info = Some((
                    session_info.name.clone(),
                    session_info.host.clone(),
                    session_info.port,
                    session_info.state.clone(),
                ));
            }
            drop(manager);

            // 显示终端界面
            self.render_terminal_session(ui, session_display_info.as_ref().unwrap());

            // 从SSH会话读取输出
            self.read_from_terminal();
        } else {
            // 显示欢迎界面
            ui.centered_and_justified(|ui| {
                ui.heading("欢迎使用 TermLink");
            });
        }
    }

    /// 渲染已连接的终端会话
    fn render_terminal_session(
        &mut self,
        ui: &mut egui::Ui,
        session_info: &(String, String, u16, SessionState),
    ) {
        let (name, host, port, state) = session_info;

        // 显示连接信息
        ui.label(format!("连接到: {}", name));
        ui.label(format!("主机: {}:{}", host, port));
        ui.label(format!("状态: {}", self.format_session_state(state)));

        // 显示终端
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                // 获取当前会话对应的终端仿真器
                if let Some(ref session_name) = self.current_session {
                    if let Some(ref mut emulator) = self.terminal_emulators.get_mut(session_name) {
                        // 更新仿真器中的主题信息
                        let theme_style = if self.settings.get_current_theme() == "light" {
                            crate::terminal::ThemeStyle::light()
                        } else {
                            crate::terminal::ThemeStyle::dark()
                        };
                        let theme = crate::terminal::TerminalTheme::new(
                            theme_style,
                            self.settings.terminal.font_size,
                        );
                        emulator.update_theme(theme.clone());

                        // 在渲染前计算并更新终端仿真器的尺寸，确保 PTY 大小与 UI 匹配
                        let font_id = egui::FontId::monospace(self.settings.terminal.font_size);
                        let galley = ui.painter().layout_no_wrap(
                            "W".to_string(),
                            font_id.clone(),
                            egui::Color32::WHITE,
                        );
                        let char_size = egui::vec2(galley.size().x, galley.size().y);

                        let available_width = ui.available_width().max(100.0);
                        let viewport_height = ui.clip_rect().height().max(100.0);

                        let actual_cols = ((available_width - 8.0) / char_size.x).floor() as usize;
                        let actual_rows =
                            ((viewport_height - 4.0) / (char_size.y * 1.2)).floor() as usize;

                        // 只有在已连接的情况下才触发 PTY 大小变化通知
                        let is_connected = matches!(state, SessionState::Connected);

                        // 如果尺寸发生了变化，则通知后端 PTY 调整
                        let current_buffer = emulator.buffer();
                        if is_connected
                            && (actual_cols != current_buffer.cols
                                || actual_rows != current_buffer.rows)
                        {
                            emulator.resize(actual_rows.max(1), actual_cols.max(1));
                        }

                        let buffer = emulator.buffer();
                        let mut renderer = crate::terminal::TerminalRenderer {
                            buffer,
                            theme,
                            font_id: egui::FontId::monospace(self.settings.terminal.font_size),
                        };

                        egui::Frame::canvas(ui.style())
                            .inner_margin(0.0)
                            .show(ui, |ui| {
                                let response = renderer.render(ui);

                                if response.clicked() {
                                    ui.memory_mut(|mem| mem.request_focus(response.id));
                                }

                                // 如果获得焦点，则处理输入和IME
                                if response.has_focus() {
                                    // 请求IME（输入法）支持，特别是中文输入
                                    ui.ctx().output_mut(|o| {
                                        // egui 0.33 IMEOutput 需要同时设置 rect 和 cursor_rect
                                        o.ime = Some(egui::output::IMEOutput {
                                            rect: response.rect,
                                            cursor_rect: response.rect,
                                        });
                                    });
                                    // 告诉egui不要处理这些按键，让它们传递给我们
                                    ui.memory_mut(|mem| {
                                        mem.set_focus_lock_filter(
                                            response.id,
                                            egui::EventFilter {
                                                tab: true,
                                                horizontal_arrows: true,
                                                vertical_arrows: true,
                                                escape: true,
                                            },
                                        );
                                        // 注意：egui::EventFilter 不包含对 Ctrl 快捷键的过滤选项
                                        // 因此我们需要在 handle_terminal_input 中通过 consume_key 来处理
                                    });
                                    self.handle_terminal_input(ui);
                                }
                            });
                    }
                }
            });
    }

    /// 处理终端输入
    fn handle_terminal_input(&mut self, ui: &mut egui::Ui) {
        let mut input_to_send = Vec::new();

        // 先检查 Context 级别的输入状态（在 egui 处理之前）
        let ctx = ui.ctx().clone();

        // 手动检查 Ctrl 组合键的按下状态
        ctx.input(|i| {
            // 检查高层事件，egui 会将 Ctrl+C/X/V 等转换成这些事件
            for event in &i.events {
                match event {
                    egui::Event::Copy => {
                        println!("[DEBUG] 拦截 Copy 事件，发送 Ctrl+C (0x03)");
                        input_to_send.push(0x03);
                    }
                    egui::Event::Cut => {
                        println!("[DEBUG] 拦截 Cut 事件，发送 Ctrl+X (0x18)");
                        input_to_send.push(0x18);
                    }
                    egui::Event::Paste(_) => {
                        // Paste 事件应该粘贴内容，不是发送 Ctrl+V
                        // 这里暂时不处理，让后续的 Text 事件处理
                        println!("[DEBUG] 拦截 Paste 事件（暂不处理，等待 Text 事件）");
                    }
                    _ => {}
                }
            }

            // 检查各个字母键是否被按下 (用于其他 Ctrl 组合键)
            if i.modifiers.ctrl && input_to_send.is_empty() {
                for (key_char, key_code, byte) in [
                    ('A', egui::Key::A, 0x01),
                    ('B', egui::Key::B, 0x02),
                    ('D', egui::Key::D, 0x04),
                    ('E', egui::Key::E, 0x05),
                    ('F', egui::Key::F, 0x06),
                    ('G', egui::Key::G, 0x07),
                    ('H', egui::Key::H, 0x08),
                    ('I', egui::Key::I, 0x09),
                    ('J', egui::Key::J, 0x0A),
                    ('K', egui::Key::K, 0x0B),
                    ('L', egui::Key::L, 0x0C),
                    ('M', egui::Key::M, 0x0D),
                    ('N', egui::Key::N, 0x0E),
                    ('O', egui::Key::O, 0x0F),
                    ('P', egui::Key::P, 0x10),
                    ('Q', egui::Key::Q, 0x11),
                    ('R', egui::Key::R, 0x12),
                    ('S', egui::Key::S, 0x13),
                    ('T', egui::Key::T, 0x14),
                    ('U', egui::Key::U, 0x15),
                    ('W', egui::Key::W, 0x17),
                    ('Y', egui::Key::Y, 0x19),
                    ('Z', egui::Key::Z, 0x1A),
                ] {
                    if i.key_pressed(key_code) {
                        println!("[DEBUG] Ctrl+{} 通过 key_pressed 检测到", key_char);
                        input_to_send.push(byte);
                        break;
                    }
                }
            }
        });

        // 使用 input_mut 以便能够消费事件
        ui.input_mut(|i| {
            // 先处理原始按键事件，检测 Ctrl 组合键
            let mut keys_to_consume = Vec::new();

            for event in &i.events {
                if let egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                    ..
                } = event
                {
                    if modifiers.ctrl {
                        // 调试日志
                        println!("[DEBUG] Ctrl 组合键检测到: {:?}", key);

                        // 记录需要消费的按键
                        keys_to_consume.push(*key);

                        // 手动映射 Ctrl 组合键
                        let code = match key {
                            egui::Key::A => Some(0x01),
                            egui::Key::B => Some(0x02),
                            egui::Key::C => Some(0x03),
                            egui::Key::D => Some(0x04),
                            egui::Key::E => Some(0x05),
                            egui::Key::F => Some(0x06),
                            egui::Key::G => Some(0x07),
                            egui::Key::H => Some(0x08),
                            egui::Key::I => Some(0x09),
                            egui::Key::J => Some(0x0A),
                            egui::Key::K => Some(0x0B),
                            egui::Key::L => Some(0x0C),
                            egui::Key::M => Some(0x0D),
                            egui::Key::N => Some(0x0E),
                            egui::Key::O => Some(0x0F),
                            egui::Key::P => Some(0x10),
                            egui::Key::Q => Some(0x11),
                            egui::Key::R => Some(0x12),
                            egui::Key::S => Some(0x13),
                            egui::Key::T => Some(0x14),
                            egui::Key::U => Some(0x15),
                            egui::Key::V => Some(0x16),
                            egui::Key::W => Some(0x17),
                            egui::Key::X => Some(0x18),
                            egui::Key::Y => Some(0x19),
                            egui::Key::Z => Some(0x1A),
                            egui::Key::OpenBracket => Some(0x1B),
                            egui::Key::Backslash => Some(0x1C),
                            egui::Key::CloseBracket => Some(0x1D),
                            egui::Key::Num6 => Some(0x1E),
                            egui::Key::Minus => Some(0x1F),
                            egui::Key::Num2 => Some(0x00), // Ctrl+@
                            _ => None,
                        };

                        if let Some(c) = code {
                            println!("[DEBUG] 发送控制码: 0x{:02X}", c);
                            input_to_send.push(c);
                        }
                    }
                }
            }

            // 消费掉 Ctrl 组合键，防止 egui 默认处理
            for key in keys_to_consume {
                i.consume_key(egui::Modifiers::CTRL, key);
            }

            // 再处理其他事件
            for event in &i.events {
                match event {
                    // 特别处理 IME 提交事件
                    egui::Event::Ime(egui::ImeEvent::Commit(text)) => {
                        println!("[DEBUG] IME Commit: {:?}", text);
                        input_to_send.extend_from_slice(text.as_bytes());
                    }
                    // 普通文本输入
                    egui::Event::Text(text) => {
                        // 调试：显示所有文本字符（包括控制字符）
                        for c in text.chars() {
                            let code = c as u32;
                            println!("[DEBUG] Text 事件字符: {:?} (0x{:02X})", c, code);

                            // 检查是否是控制字符
                            if code <= 0x1F {
                                println!("[DEBUG] 检测到控制字符，直接发送: 0x{:02X}", code);
                                input_to_send.push(code as u8);
                            } else {
                                let mut b = [0; 4];
                                input_to_send.extend_from_slice(c.encode_utf8(&mut b).as_bytes());
                            }
                        }
                    }
                    // 按键事件（非 Ctrl 组合键）
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        println!(
                            "[DEBUG] Key 事件: {:?}, Ctrl={}, Alt={}, Shift={}",
                            key, modifiers.ctrl, modifiers.alt, modifiers.shift
                        );

                        if !modifiers.ctrl && modifiers.alt {
                            // 处理 Alt 组合键 (Meta 键支持)
                            if let Some(mut bytes) = self.key_to_bytes(*key) {
                                input_to_send.push(0x1b); // 发送 ESC 前缀
                                input_to_send.append(&mut bytes);
                            }
                        } else if !modifiers.ctrl && !modifiers.alt {
                            // 普通功能按键 (Tab, ArrowKeys, Backspace, Home, End, F1-F12等)
                            if let Some(key_code) = self.key_to_bytes(*key) {
                                input_to_send.extend(key_code);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        // 发送收集到的输入
        if !input_to_send.is_empty() {
            println!(
                "[DEBUG] 发送 {} 字节到终端: {:?}",
                input_to_send.len(),
                input_to_send
            );
            self.send_to_terminal(&input_to_send);
        }
    }

    /// 连接配置对话框
    fn connection_dialog(&mut self, ctx: &egui::Context) {
        let title = if self.editing_connection_name.is_some() {
            self.i18n.get(I18nKey::EditConnection)
        } else {
            self.i18n.get(I18nKey::NewConnection)
        };

        egui::Window::new(title)
            .default_width(350.0)
            .default_height(450.0)
            .collapsible(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(self.i18n.get(I18nKey::ConnectionName));
                        ui.text_edit_singleline(&mut self.connection_form.name);

                        ui.label(self.i18n.get(I18nKey::HostAddress));
                        ui.text_edit_singleline(&mut self.connection_form.host);

                        ui.label(self.i18n.get(I18nKey::Port));
                        ui.add(egui::DragValue::new(&mut self.connection_form.port).speed(1));

                        ui.label(self.i18n.get(I18nKey::Username));
                        ui.text_edit_singleline(&mut self.connection_form.username);

                        ui.label(self.i18n.get(I18nKey::AuthMethod));
                        ui.horizontal(|ui| {
                            ui.radio_value(
                                &mut self.connection_form.use_key_auth,
                                false,
                                self.i18n.get(I18nKey::Password),
                            );
                            ui.radio_value(
                                &mut self.connection_form.use_key_auth,
                                true,
                                self.i18n.get(I18nKey::PrivateKey),
                            );
                        });

                        if self.connection_form.use_key_auth {
                            ui.label(self.i18n.get(I18nKey::PrivateKeyPath));
                            ui.horizontal(|ui| {
                                ui.text_edit_singleline(&mut self.connection_form.private_key_path);
                                if ui.button(self.i18n.get(I18nKey::Browse)).clicked() {
                                    // TODO: 打开文件选择对话框
                                }
                            });
                        } else {
                            ui.label(self.i18n.get(I18nKey::PasswordLabel));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.connection_form.password)
                                    .password(true),
                            );
                        }

                        ui.checkbox(
                            &mut self.connection_form.save_to_history,
                            self.i18n.get(I18nKey::SaveToHistory),
                        );

                        // 分组选择
                        ui.label(self.i18n.get(I18nKey::Group));
                        egui::ComboBox::from_label(self.i18n.get(I18nKey::SelectGroup))
                            .selected_text(
                                self.connection_form
                                    .group
                                    .as_ref()
                                    .map(|g| g.as_str())
                                    .unwrap_or(self.i18n.get(I18nKey::NoGroup)),
                            )
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.connection_form.group,
                                    None,
                                    self.i18n.get(I18nKey::NoGroup),
                                );
                                for group in &self.connection_groups {
                                    ui.selectable_value(
                                        &mut self.connection_form.group,
                                        Some(group.name.clone()),
                                        &group.name,
                                    );
                                }
                            });

                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button(self.i18n.get(I18nKey::QuickConnect)).clicked() {
                                self.quick_connect();
                            }
                            if ui.button(self.i18n.get(I18nKey::TestConnection)).clicked() {
                                self.test_connection();
                            }
                            if ui.button(self.i18n.get(I18nKey::SaveToGroup)).clicked() {
                                self.save_to_group();
                            }
                        });

                        ui.separator();

                        ui.horizontal(|ui| {
                            if ui.button(self.i18n.get(I18nKey::Connect)).clicked() {
                                self.connect_to_host();
                            }
                            if ui.button(self.i18n.get(I18nKey::Cancel)).clicked() {
                                self.show_connection_dialog = false;
                                self.editing_connection_name = None;
                                // 重置表单
                                self.connection_form = ConnectionForm::default();
                            }
                        });
                    });
                });
            });
    }

    /// 快速连接
    fn quick_connect(&mut self) {
        // 使用表单数据进行快速连接，不显示完整对话框
        if self.connection_form.host.is_empty() || self.connection_form.username.is_empty() {
            return;
        }

        self.connect_to_host();
    }

    /// 测试连接
    fn test_connection(&mut self) {
        // TODO: 实现连接测试功能
        println!("测试连接功能待实现");
    }

    /// 保存到分组
    fn save_to_group(&mut self) {
        if let Some(ref group_name) = self.connection_form.group {
            let group_name_clone = group_name.clone();

            // 创建连接配置但不立即连接
            let connection_name = if self.connection_form.name.is_empty() {
                format!(
                    "{}@{}",
                    self.connection_form.username, self.connection_form.host
                )
            } else {
                self.connection_form.name.clone()
            };

            let config = ConnectionConfig {
                name: connection_name,
                host: self.connection_form.host.clone(),
                port: self.connection_form.port,
                username: self.connection_form.username.clone(),
                use_key_auth: self.connection_form.use_key_auth,
                private_key_path: if self.connection_form.use_key_auth {
                    Some(self.connection_form.private_key_path.clone())
                } else {
                    None
                },
                // 保存实际密码（不再依赖复杂的占位符检查）
                password: if !self.connection_form.password.is_empty() {
                    Some(self.connection_form.password.clone())
                } else {
                    None
                },
                last_connected: None,
                group: Some(group_name_clone.clone()),
            };

            // 处理更新或添加
            if let Some(old_name) = self.editing_connection_name.take() {
                // 如果名称发生了改变，清理旧名称在分组中的引用
                if old_name != config.name {
                    for group in self.connection_groups.iter_mut() {
                        group.connections.retain(|n| n != &old_name);
                    }
                }

                if let Some(existing) = self
                    .connection_history
                    .iter_mut()
                    .find(|c| c.name == old_name)
                {
                    *existing = config.clone();
                } else if !self
                    .connection_history
                    .iter()
                    .any(|c| c.name == config.name)
                {
                    self.connection_history.push(config.clone());
                }
            } else {
                // 添加到连接历史（如果不存在同名）
                if let Some(existing) = self
                    .connection_history
                    .iter_mut()
                    .find(|c| c.name == config.name)
                {
                    *existing = config.clone();
                } else {
                    self.connection_history.push(config.clone());
                }
            }

            // 添加到分组
            self.add_connection_to_group(&config.name, &group_name_clone);

            // 自动保存状态
            self.auto_save_state();

            // 关闭对话框
            self.show_connection_dialog = false;
            // 重置表单
            self.connection_form = ConnectionForm::default();

            println!("连接 '{}' 已保存到分组 '{}'", config.name, group_name_clone);
        }
    }

    /// 从历史记录连接
    fn connect_from_history(&mut self, config: ConnectionConfig) {
        // 填充表单数据
        self.connection_form = ConnectionForm {
            name: config.name.clone(),
            host: config.host.clone(),
            port: config.port,
            username: config.username.clone(),
            // 使用保存的密码，如果为空则设为空字符串，以便触发重新输入逻辑
            password: config.password.clone().unwrap_or_default(),
            use_key_auth: config.use_key_auth,
            private_key_path: config.private_key_path.unwrap_or_default(),
            save_to_history: true,
            group: config.group.clone(),
        };

        // 直接连接，不需要显示对话框
        self.direct_connect();
    }

    /// 直接连接（使用表单数据）
    fn direct_connect(&mut self) {
        // 检查密码是否需要重新输入
        if !self.connection_form.use_key_auth && self.connection_form.password.is_empty() {
            // 显示连接对话框让用户输入密码
            self.show_connection_dialog = true;
        } else {
            // 密码已存在或使用密钥认证，直接连接
            self.connect_to_host();
        }
    }

    /// 编辑连接
    fn edit_connection(&mut self, config: ConnectionConfig) {
        self.editing_connection_name = Some(config.name.clone());
        self.connection_form = ConnectionForm {
            name: config.name.clone(),
            host: config.host.clone(),
            port: config.port,
            username: config.username.clone(),
            // 编辑时显示真实密码（如果是密码认证且已保存），UI 会通过 password 遮罩保护它
            password: config.password.clone().unwrap_or_default(),
            use_key_auth: config.use_key_auth,
            private_key_path: config.private_key_path.unwrap_or_default(),
            save_to_history: true,
            group: config.group.clone(),
        };

        self.show_connection_dialog = true;
    }

    /// 新建/编辑分组对话框
    fn create_group_dialog(&mut self, ctx: &egui::Context) {
        let title = if self.editing_group_index.is_some() {
            self.i18n.get(I18nKey::EditGroup)
        } else {
            self.i18n.get(I18nKey::CreateGroup)
        };

        egui::Window::new(title)
            .default_width(300.0)
            .default_height(200.0)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(self.i18n.get(I18nKey::GroupName));
                    ui.text_edit_singleline(&mut self.group_form.name);

                    ui.label(self.i18n.get(I18nKey::GroupDescription));
                    ui.text_edit_multiline(&mut self.group_form.description);

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui
                            .button(if self.editing_group_index.is_some() {
                                self.i18n.get(I18nKey::Save)
                            } else {
                                self.i18n.get(I18nKey::Create)
                            })
                            .clicked()
                        {
                            self.save_group();
                        }
                        if ui.button(self.i18n.get(I18nKey::Cancel)).clicked() {
                            self.show_create_group_dialog = false;
                            self.editing_group_index = None;
                            self.group_form = GroupForm::default(); // 重置表单
                        }
                    });
                });
            });
    }

    /// 保存分组（新建或更新）
    fn save_group(&mut self) {
        if !self.group_form.name.trim().is_empty() {
            let name = self.group_form.name.trim().to_string();
            let description = if self.group_form.description.trim().is_empty() {
                None
            } else {
                Some(self.group_form.description.trim().to_string())
            };

            if let Some(index) = self.editing_group_index {
                // 编辑现有分组
                let old_name = self.connection_groups[index].name.clone();
                self.connection_groups[index].name = name.clone();
                self.connection_groups[index].description = description;

                // 如果名称改变，更新所有引用该分组名称的连接
                if old_name != name {
                    for config in self.connection_history.iter_mut() {
                        if config.group == Some(old_name.clone()) {
                            config.group = Some(name.clone());
                        }
                    }
                }
            } else {
                // 创建新分组
                if !self.connection_groups.iter().any(|g| g.name == name) {
                    self.connection_groups.push(ConnectionGroup {
                        name,
                        description,
                        connections: Vec::new(),
                    });
                } else {
                    println!("分组名称 '{}' 已存在", name);
                    return;
                }
            }

            self.show_create_group_dialog = false;
            self.editing_group_index = None;
            self.group_form = GroupForm::default();
            self.auto_save_state();
        }
    }

    /// 编辑分组
    fn edit_group(&mut self, index: usize) {
        if let Some(group) = self.connection_groups.get(index) {
            self.group_form = GroupForm {
                name: group.name.clone(),
                description: group.description.clone().unwrap_or_default(),
            };
            self.editing_group_index = Some(index);
            self.show_create_group_dialog = true;
        }
    }

    /// 删除分组
    fn delete_group(&mut self, index: usize) {
        if index < self.connection_groups.len() {
            let group_name = self.connection_groups[index].name.clone();
            self.connection_groups.remove(index);

            // 清理连接历史中引用该分组的信息
            for config in self.connection_history.iter_mut() {
                if config.group == Some(group_name.clone()) {
                    config.group = None;
                }
            }

            self.auto_save_state();
        }
    }

    /// 从分组中移除连接
    fn remove_connection_from_group(&mut self, group_index: usize, conn_name: &str) {
        if group_index < self.connection_groups.len() {
            self.connection_groups[group_index]
                .connections
                .retain(|name| name != conn_name);

            // 同时将历史配置中的分组设为 None
            if let Some(config) = self
                .connection_history
                .iter_mut()
                .find(|c| c.name == conn_name)
            {
                config.group = None;
            }

            self.auto_save_state();
        }
    }

    /// 添加连接到分组
    fn add_connection_to_group(&mut self, conn_name: &str, group_name: &str) {
        // 提取基础连接名称（去除实例后缀）
        let base_name = self.extract_base_connection_name(conn_name);

        // 如果连接已经在其他中心化分组中，先从旧分组删除
        for group in self.connection_groups.iter_mut() {
            group.connections.retain(|name| name != &base_name);
        }

        // 添加到新分组
        if let Some(group) = self
            .connection_groups
            .iter_mut()
            .find(|g| g.name == group_name)
        {
            if !group.connections.contains(&base_name) {
                group.connections.push(base_name.clone());
            }
        }

        // 更新连接配置中的分组信息
        if let Some(config) = self
            .connection_history
            .iter_mut()
            .find(|c| c.name == base_name)
        {
            config.group = Some(group_name.to_string());
        }
    }

    /// 从分组中移除连接（按名称）
    fn remove_connection_from_group_by_name(&mut self, conn_name: &str, group_name: &str) {
        if let Some(group) = self
            .connection_groups
            .iter_mut()
            .find(|g| g.name == group_name)
        {
            group.connections.retain(|name| name != conn_name);
        }

        // 更新连接配置中的分组信息
        if let Some(config) = self
            .connection_history
            .iter_mut()
            .find(|c| c.name == conn_name)
        {
            config.group = None;
        }

        self.auto_save_state();
    }

    /// 清除单个连接的连接时间记录，即从“最近”中移除
    fn clear_connection_history(&mut self, index: usize) {
        if index < self.connection_history.len() {
            self.connection_history[index].last_connected = None;
            self.auto_save_state();
        }
    }

    /// 删除单个连接配置
    fn delete_connection(&mut self, index: usize) {
        if index < self.connection_history.len() {
            let config = self.connection_history.remove(index);

            // 如果该连接在某个分组中，同步清理分组信息
            if let Some(ref group_name) = config.group {
                if let Some(group) = self
                    .connection_groups
                    .iter_mut()
                    .find(|g| g.name == *group_name)
                {
                    group.connections.retain(|name| name != &config.name);
                }
            }

            self.auto_save_state();
        }
    }

    /// 从分组连接
    fn connect_from_group(&mut self, _group_index: usize, conn_name: &str) {
        // 从连接历史中找到对应的连接配置
        if let Some(config) = self
            .connection_history
            .iter()
            .find(|c| c.name == conn_name)
            .cloned()
        {
            self.connect_from_history(config);
        }
    }

    /// 从分组编辑连接
    fn edit_connection_from_group(&mut self, _group_index: usize, conn_name: &str) {
        // 从连接历史中找到对应的连接配置
        if let Some(config) = self
            .connection_history
            .iter()
            .find(|c| c.name == conn_name)
            .cloned()
        {
            self.edit_connection(config);
        }
    }

    /// 生成唯一的会话名称
    fn generate_unique_session_name(&self, base_name: &str) -> String {
        let manager = self.connection_manager.lock().unwrap();
        let active_sessions = manager.get_active_sessions();
        drop(manager);

        // 如果没有同名会话，直接使用基础名称
        if !active_sessions
            .iter()
            .any(|name| name.starts_with(base_name))
        {
            return base_name.to_string();
        }

        // 查找可用的序号
        let mut counter = 1;
        loop {
            let session_name = format!("{} #{}", base_name, counter);
            if !active_sessions.contains(&session_name) {
                return session_name;
            }
            counter += 1;
        }
    }

    /// 从会话名称中提取基础连接名称
    fn extract_base_connection_name(&self, session_name: &str) -> String {
        // 检查是否包含实例后缀（如 "server #1"）
        if let Some(pos) = session_name.rfind(" #") {
            // 确保后面是数字
            let suffix = &session_name[pos + 2..];
            if suffix.chars().all(|c| c.is_ascii_digit()) {
                return session_name[..pos].to_string();
            }
        }
        // 没有后缀，返回原名称
        session_name.to_string()
    }

    /// 渲染会话标签页
    fn render_session_tabs(&mut self, ui: &mut egui::Ui) {
        let manager = self.connection_manager.lock().unwrap();
        let active_sessions = manager.get_active_sessions();
        drop(manager); // 释放锁

        if !active_sessions.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for session_name in active_sessions {
                    let is_selected = self.current_session.as_ref() == Some(&session_name);

                    ui.horizontal(|ui| {
                        if ui.selectable_label(is_selected, &session_name).clicked() {
                            self.current_session = Some(session_name.clone());
                        }

                        // 添加关闭按钮
                        if ui.small_button("x").clicked() {
                            self.close_session(&session_name);
                        }
                    });

                    ui.separator();
                }
            });
            ui.separator();
        } else {
            ui.label("暂无活动连接");
            ui.separator();
        }
    }

    /// 连接到主机
    fn connect_to_host(&mut self) {
        // 1. 获取表单中的原始信息进行备份
        let is_use_key_auth = self.connection_form.use_key_auth;
        let form_password = if !is_use_key_auth {
            // 如果由于某种原因密码字段为空，则视为没有密码
            Some(self.connection_form.password.clone())
        } else {
            None
        };
        let should_save_to_history = self.connection_form.save_to_history;
        let selected_group = self.connection_form.group.clone();

        // 2. 生成基础连接名称（用于配置档案）
        let base_name = if self.connection_form.name.is_empty() {
            format!(
                "{}@{}",
                self.connection_form.username, self.connection_form.host
            )
        } else {
            self.connection_form.name.clone()
        };

        // 3. 生成唯一的会话名称（用于当前活动的连接实例）
        let session_name = self.generate_unique_session_name(&base_name);

        // 4. 创建永久档案配置
        let config = ConnectionConfig {
            name: base_name.clone(), // 档案名使用基础名，不带后缀
            host: self.connection_form.host.clone(),
            port: self.connection_form.port,
            username: self.connection_form.username.clone(),
            use_key_auth: is_use_key_auth,
            private_key_path: if is_use_key_auth {
                Some(self.connection_form.private_key_path.clone())
            } else {
                None
            },
            password: if let Some(ref p) = form_password {
                if !p.is_empty() { Some(p.clone()) } else { None }
            } else {
                None
            },
            last_connected: Some(chrono::Local::now().to_rfc3339()),
            group: selected_group.clone(),
        };

        // 5. 设置当前活跃会话和 UI 状态
        let manager = self.connection_manager.lock().unwrap();
        let session_exists = manager.is_session_active(&session_name);
        drop(manager);

        if !session_exists {
            self.current_session = Some(session_name.clone());
        }
        self.show_connection_dialog = false;

        // 6. 后台连接准备工作
        let password_for_thread = if let Some(ref p) = form_password {
            if !p.is_empty() { Some(p.clone()) } else { None }
        } else {
            None
        };

        // 7. 重置表单（数据已备份到 config 和局部变量）
        self.connection_form = ConnectionForm::default();

        // 8. 初始化终端仿真器
        let mut emulator = TerminalEmulator::new(40, 120);

        // 设置终端事件回调
        let session_name_clone = session_name.clone();
        let manager_clone = self.connection_manager.clone();
        emulator.set_event_callback(move |event| {
            if let crate::terminal::TerminalEvent::Resize { rows, cols } = event {
                let manager = manager_clone.lock().unwrap();
                if let Some(session) = manager.get_session(&session_name_clone) {
                    if let Err(e) = session.resize_terminal(rows as u32, cols as u32) {
                        eprintln!("调整终端大小失败: {}", e);
                    }
                }
            }
        });

        self.terminal_emulators
            .insert(session_name.clone(), emulator);

        // 9. 在后台线程中执行连接
        let manager_arc = self.connection_manager.clone();
        let config_clone = config.clone();
        let session_name_clone = session_name.clone();

        // 首先在 manager 中创建一个占位符
        {
            let mut manager = manager_arc.lock().unwrap();
            // 注意：manager 里的 configs 是档案列表，session 是实例列表
            manager.add_connection_config(config_clone.clone());

            let mut session = SshSession::new(
                session_name_clone.clone(),
                config_clone.host.clone(),
                config_clone.port,
            );
            session.state = crate::ssh::SessionState::Connecting;
            manager.add_session(session_name_clone.clone(), session);
        }

        std::thread::spawn(move || {
            let mut session = SshSession::new(
                session_name_clone.clone(),
                config_clone.host.clone(),
                config_clone.port,
            );

            println!("开始连接会话: {}", session_name_clone);
            match session.connect(
                &config_clone.username,
                password_for_thread.as_deref(),
                config_clone.private_key_path.as_deref(),
            ) {
                Ok(_) => {
                    println!("会话连接成功: {}", session_name_clone);
                    let manager = manager_arc.lock().unwrap();
                    manager.add_session(session_name_clone, session);
                }
                Err(e) => {
                    eprintln!("会话连接失败: {}", e);
                    let manager = manager_arc.lock().unwrap();
                    let mut err_session = SshSession::new(
                        session_name_clone.clone(),
                        config_clone.host.clone(),
                        config_clone.port,
                    );
                    err_session.state = crate::ssh::SessionState::Error(e.to_string());
                    manager.add_session(session_name_clone, err_session);
                }
            }
        });

        // 10. 持久化到应用级别的历史记录
        if should_save_to_history {
            if let Some(old_name) = self.editing_connection_name.take() {
                // 如果名称发生了改变，清理旧名称在分组中的引用
                if old_name != config.name {
                    for group in self.connection_groups.iter_mut() {
                        group.connections.retain(|n| n != &old_name);
                    }
                }

                // 如果是编辑现有连接
                if let Some(existing) = self
                    .connection_history
                    .iter_mut()
                    .find(|c| c.name == old_name)
                {
                    *existing = config.clone();
                } else if !self
                    .connection_history
                    .iter()
                    .any(|c| c.name == config.name)
                {
                    self.connection_history.push(config.clone());
                }
            } else {
                // 如果是新连接，检查历史记录中是否已有同名配置
                if let Some(existing) = self
                    .connection_history
                    .iter_mut()
                    .find(|c| c.name == config.name)
                {
                    *existing = config.clone();
                } else {
                    self.connection_history.push(config.clone());
                }
            }

            if let Some(ref group_name) = selected_group {
                self.add_connection_to_group(&config.name, group_name);
            }

            self.auto_save_state();
        } else {
            self.editing_connection_name = None;
        }
    }

    /// 应用设置变更
    fn apply_settings(&mut self, new_settings: AppSettings, new_language: Language) {
        // 更新语言
        self.i18n.set_language(new_language);

        // 更新设置
        self.settings = new_settings;

        // 保存到文件
        self.auto_save_state();

        println!("设置已更新");
    }

    /// 格式化会话状态显示
    fn format_session_state(&self, state: &SessionState) -> String {
        match state {
            SessionState::Disconnected => self.i18n.get(I18nKey::Disconnected).to_string(),
            SessionState::Connecting => self.i18n.get(I18nKey::Connecting).to_string(),
            SessionState::Connected => self.i18n.get(I18nKey::Connected).to_string(),
            SessionState::Error(e) => format!("{}: {}", self.i18n.get(I18nKey::ConnectionError), e),
        }
    }

    /// 发送数据到终端
    fn send_to_terminal(&mut self, data: &[u8]) {
        if let Some(ref session_name) = self.current_session {
            let manager = self.connection_manager.lock().unwrap();
            if let Some(session) = manager.get_session(session_name) {
                if session.is_connected() {
                    if let Err(e) = session.write_terminal(data) {
                        eprintln!("[TermLink] 发送数据失败: {}", e);
                    }
                }
            }
        }
    }

    /// 从终端读取数据
    fn read_from_terminal(&mut self) {
        // 移除轮询限制，让系统自然处理
        // 但增加错误处理和非阻塞检查

        if let Some(ref session_name) = self.current_session {
            let manager = self.connection_manager.lock().unwrap();
            if let Some(session) = manager.get_session(session_name) {
                // 只在确定可读时才尝试读取
                if session.is_terminal_readable() {
                    let mut buffer = [0u8; 256]; // 进一步减少缓冲区
                    match session.read_terminal(&mut buffer) {
                        Ok(n) if n > 0 => {
                            // 获取当前会话对应的终端仿真器
                            if let Some(ref session_name) = self.current_session {
                                if let Some(ref mut emulator) =
                                    self.terminal_emulators.get_mut(session_name)
                                {
                                    if let Err(e) = emulator.process_input(&buffer[..n]) {
                                        eprintln!("处理终端输入失败: {}", e);
                                    }
                                }
                            }
                        }
                        Ok(_) => {
                            // 没有数据可读，正常情况
                        }
                        Err(e) => {
                            // 对于阻塞错误，静默处理
                            if !e.to_string().contains("would block")
                                && !e.to_string().contains("timed out")
                            {
                                eprintln!("读取终端数据失败: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    /// 保存应用状态到本地文件
    fn save_app_state(&self) {
        // 更新设置中的连接历史和分组
        let mut settings = self.settings.clone();
        settings.connections = self.connection_history.clone();
        settings.groups = self.connection_groups.clone();

        // 保存到文件
        if let Err(e) = settings.save() {
            eprintln!("保存应用状态失败: {}", e);
        } else {
            println!("应用状态已保存");
        }
    }

    /// 从本地文件加载应用状态
    fn load_app_state(&mut self) {
        match AppSettings::load() {
            Ok(settings) => {
                self.settings = settings.clone();
                self.connection_history = settings.connections;
                self.connection_groups = settings.groups;
                println!("应用状态已加载");
            }
            Err(e) => {
                eprintln!("加载应用状态失败: {}", e);
                // 使用默认设置
                self.settings = AppSettings::default();
                self.connection_history = Vec::new();
                self.connection_groups = Vec::new();
            }
        }
    }

    /// 在关键操作后自动保存状态
    fn auto_save_state(&self) {
        self.save_app_state();
    }

    /// 将按键转换为字节序列
    fn key_to_bytes(&self, key: egui::Key) -> Option<Vec<u8>> {
        match key {
            egui::Key::Enter => Some(b"\r".to_vec()),
            egui::Key::Backspace => Some(b"\x7f".to_vec()),
            egui::Key::Tab => Some(b"\t".to_vec()),
            egui::Key::Escape => Some(b"\x1b".to_vec()),
            egui::Key::ArrowUp => Some(b"\x1b[A".to_vec()),
            egui::Key::ArrowDown => Some(b"\x1b[B".to_vec()),
            egui::Key::ArrowLeft => Some(b"\x1b[D".to_vec()),
            egui::Key::ArrowRight => Some(b"\x1b[C".to_vec()),
            egui::Key::Delete => Some(b"\x1b[3~".to_vec()),
            egui::Key::Insert => Some(b"\x1b[2~".to_vec()),
            egui::Key::Home => Some(b"\x1b[H".to_vec()),
            egui::Key::End => Some(b"\x1b[F".to_vec()),
            egui::Key::PageUp => Some(b"\x1b[5~".to_vec()),
            egui::Key::PageDown => Some(b"\x1b[6~".to_vec()),
            _ => None,
        }
    }

    /// 更新所有终端的主题
    fn update_terminal_themes(&mut self) {
        let theme_style = if self.settings.get_current_theme() == "light" {
            crate::terminal::ThemeStyle::light()
        } else {
            crate::terminal::ThemeStyle::dark()
        };
        let theme =
            crate::terminal::TerminalTheme::new(theme_style, self.settings.terminal.font_size);

        // 更新所有已存在的终端仿真器主题
        for (_, emulator) in self.terminal_emulators.iter_mut() {
            emulator.update_theme(theme.clone());
        }
    }
}
