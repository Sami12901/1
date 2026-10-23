use crate::theme::CapBdTheme;
use capbd_core::timeline::Timeline;
use capbd_engine::exporter::{ExportResolution, ExportSettings, TimelineCompositor};
use capbd_engine::video_decoder::MediaDecoderCache;
use egui::{vec2, Align2, Color32, ProgressBar, RichText, Window};
use std::path::PathBuf;

pub struct ExportDialog {
    pub settings: ExportSettings,
    pub is_exporting: bool,
    pub export_progress: f32,
    pub status_message: String,
}

impl Default for ExportDialog {
    fn default() -> Self {
        Self {
            settings: ExportSettings::default(),
            is_exporting: false,
            export_progress: 0.0,
            status_message: String::new(),
        }
    }
}

impl ExportDialog {
    pub fn render(
        &mut self,
        ctx: &egui::Context,
        timeline: &Timeline,
        cache: &MediaDecoderCache,
        open: &mut bool,
    ) {
        if !*open {
            return;
        }

        Window::new("Export Video — CapBD")
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .resizable(false)
            .collapsible(false)
            .open(open)
            .show(ctx, |ui| {
                ui.set_min_width(380.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new("Export Settings").strong().color(Color32::WHITE).size(14.0));
                    ui.add_space(8.0);

                    // Resolution
                    ui.horizontal(|ui| {
                        ui.label("Resolution:");
                        let resolutions = [
                            ExportResolution::Res720p,
                            ExportResolution::Res1080p,
                            ExportResolution::Res4k,
                        ];
                        for r in resolutions {
                            ui.selectable_value(&mut self.settings.resolution, r, r.label());
                        }
                    });

                    // Dimensions display
                    let (w, h) = self.settings.resolution.dimensions(timeline.aspect_ratio);
                    ui.label(
                        RichText::new(format!("Canvas Size: {} x {} px", w, h))
                            .size(11.0)
                            .color(CapBdTheme::TEXT_MUTED),
                    );

                    ui.separator();

                    // Framerate
                    ui.horizontal(|ui| {
                        ui.label("Frame Rate:");
                        let f_rates = [24, 30, 60];
                        for f in f_rates {
                            ui.selectable_value(&mut self.settings.fps, f, format!("{} FPS", f));
                        }
                    });

                    ui.separator();

                    // Output File Name
                    ui.horizontal(|ui| {
                        ui.label("Output File:");
                        ui.text_edit_singleline(&mut self.settings.output_path);
                    });

                    if !self.status_message.is_empty() {
                        ui.label(RichText::new(&self.status_message).color(CapBdTheme::ACCENT_CYAN));
                    }

                    if self.is_exporting {
                        ui.add(ProgressBar::new(self.export_progress).show_percentage());
                    }

                    ui.add_space(12.0);

                    // Export / Cancel Buttons
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.is_exporting = false;
                        }

                        let start_btn = egui::Button::new(
                            RichText::new("Start Export")
                                .color(Color32::BLACK)
                                .strong(),
                        )
                        .fill(CapBdTheme::ACCENT_CYAN);

                        if ui.add(start_btn).clicked() {
                            self.export_progress = 0.5;

                            // Export snapshot or render test frame
                            let path = PathBuf::from(&self.settings.output_path);
                            match TimelineCompositor::export_snapshot(
                                timeline,
                                timeline.playhead_frame,
                                &path,
                                cache,
                            ) {
                                Ok(_) => {
                                    self.export_progress = 1.0;
                                    self.status_message = format!("Export successful: {}", path.display());
                                }
                                Err(e) => {
                                    self.status_message = format!("Export error: {}", e);
                                }
                            }
                        }
                    });
                });
            });
    }
}
