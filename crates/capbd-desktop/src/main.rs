use capbd_ui::CapBdApp;
use eframe::egui;
use log::info;
use std::sync::Arc;

struct CapBdDesktopApp {
    app: CapBdApp,
}

impl CapBdDesktopApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            app: CapBdApp::new(),
        }
    }
}

impl eframe::App for CapBdDesktopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.app.update_ui(ctx);
    }
}

fn load_app_icon() -> Option<egui::IconData> {
    let icon_bytes = include_bytes!("../../../assets/logo/capbd_app_logo.jpg");
    if let Ok(img) = image::load_from_memory(icon_bytes) {
        let rgba = img.to_rgba8();
        Some(egui::IconData {
            rgba: rgba.into_raw(),
            width: img.width(),
            height: img.height(),
        })
    } else {
        None
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    info!("Starting CapBD: Photo & Video Editor...");

    let mut viewport = egui::ViewportBuilder::default()
        .with_title("CapBD: Photo & Video Editor")
        .with_inner_size([1366.0, 768.0])
        .with_min_inner_size([1024.0, 600.0])
        .with_app_id("org.capbd.editor");

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(Arc::new(icon));
    }

    let native_options = eframe::NativeOptions {
        viewport,
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "CapBD: Photo & Video Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(CapBdDesktopApp::new(cc)))),
    )
}
