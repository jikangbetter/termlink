use eframe::egui;

// 引入项目模块
mod app;
mod config;
mod i18n;
mod sftp;
mod ssh;
mod terminal;
mod utils;

use app::App;

/// 设置中文字体
pub fn setup_chinese_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 尝试从本地加载字体文件
    #[cfg(not(target_arch = "wasm32"))]
    let font_loaded = {
        let font_path = "fonts/方正黑体简体.ttf";
        if let Ok(font_data) = std::fs::read(font_path) {
            fonts.font_data.insert(
                "chinese_font".to_owned(),
                std::sync::Arc::new(egui::FontData::from_owned(font_data)),
            );
            true
        } else {
            // 如果指定字体不存在，尝试使用系统默认中文字体
            try_system_chinese_fonts(&mut fonts)
        }
    };

    // Web环境下使用默认字体
    #[cfg(target_arch = "wasm32")]
    let font_loaded = false;

    if font_loaded {
        // 将字体族添加到family列表
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "chinese_font".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("chinese_font".to_owned());
    }

    ctx.set_fonts(fonts);
}

/// 尝试使用系统默认中文字体
fn try_system_chinese_fonts(fonts: &mut egui::FontDefinitions) -> bool {
    // Windows系统中文字体路径
    let windows_fonts = [
        "C:/Windows/Fonts/msyh.ttc",   // 微软雅黑
        "C:/Windows/Fonts/simhei.ttf", // 黑体
        "C:/Windows/Fonts/simsun.ttc", // 宋体
    ];

    for font_path in &windows_fonts {
        if let Ok(font_data) = std::fs::read(font_path) {
            fonts.font_data.insert(
                "chinese_font".to_owned(),
                std::sync::Arc::new(egui::FontData::from_owned(font_data)),
            );
            return true;
        }
    }

    false
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // 初始化日志

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "TermLink",
        options,
        Box::new(|cc| {
            setup_chinese_font(&cc.egui_ctx);
            Ok(Box::new(App::default()))
        }),
    )
}
