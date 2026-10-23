use crate::theme::CapBdTheme;
use capbd_core::clip::{Clip, MediaKind, TestPattern};
use capbd_core::timeline::Timeline;
use egui::{vec2, Color32, RichText, ScrollArea, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaTab {
    Media,
    Text,
    Audio,
    Filters,
}

pub struct MediaPanel {
    pub active_tab: MediaTab,
}

impl Default for MediaPanel {
    fn default() -> Self {
        Self {
            active_tab: MediaTab::Media,
        }
    }
}

impl MediaPanel {
    pub fn render(&mut self, ui: &mut Ui, timeline: &mut Timeline) {
        ui.vertical(|ui| {
            // Navigation Tabs
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, MediaTab::Media, "Media");
                ui.selectable_value(&mut self.active_tab, MediaTab::Text, "Text");
                ui.selectable_value(&mut self.active_tab, MediaTab::Audio, "Audio");
                ui.selectable_value(&mut self.active_tab, MediaTab::Filters, "Filters");
            });

            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                match self.active_tab {
                    MediaTab::Media => Self::render_media_tab(ui, timeline),
                    MediaTab::Text => Self::render_text_tab(ui, timeline),
                    MediaTab::Audio => Self::render_audio_tab(ui, timeline),
                    MediaTab::Filters => Self::render_filters_tab(ui, timeline),
                }
            });
        });
    }

    fn render_media_tab(ui: &mut Ui, timeline: &mut Timeline) {
        // Import Button
        if ui
            .add_sized(
                [ui.available_width(), 32.0],
                egui::Button::new(
                    RichText::new("Import Media...")
                        .color(Color32::WHITE)
                        .strong(),
                )
                .fill(CapBdTheme::BG_CARD),
            )
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Media Files", &["mp4", "mov", "mkv", "png", "jpg", "jpeg", "webp"])
                .pick_file()
            {
                let path_str = path.to_string_lossy().to_string();
                let file_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Media Clip".into());

                let start = timeline.playhead_frame;
                let clip = Clip::new_file(file_name, MediaKind::Video, path_str, start, 150);
                timeline.add_clip_to_best_track(clip);
            }
        }

        ui.add_space(8.0);
        ui.label(RichText::new("Built-in Generators").color(CapBdTheme::TEXT_SECONDARY).size(11.0));

        let presets = [
            ("SMPTE Bars", TestPattern::SmpteColorBars, 150),
            ("Neon Motion", TestPattern::NeonGradientMotion, 180),
            ("Countdown", TestPattern::CountdownLeader, 240),
            ("Cyber Grid", TestPattern::CyberGrid, 150),
            ("Oscilloscope", TestPattern::AudioWaveformPulse, 120),
        ];

        for (label, pattern, duration) in presets {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(label).strong().color(Color32::WHITE));
                        ui.label(
                            RichText::new(format!("{:.1}s @ 30fps", duration as f32 / 30.0))
                                .size(11.0)
                                .color(CapBdTheme::TEXT_MUTED),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("+ Add").clicked() {
                            let start = timeline.playhead_frame;
                            let clip = Clip::new_synthetic(label, MediaKind::Video, pattern, start, duration);
                            timeline.add_clip_to_best_track(clip);
                        }
                    });
                });
            });
        }
    }

    fn render_text_tab(ui: &mut Ui, timeline: &mut Timeline) {
        ui.label(RichText::new("Title & Text Presets").color(CapBdTheme::TEXT_SECONDARY).size(11.0));

        let text_presets = [
            "CapBD Masterpiece",
            "Follow for More",
            "Chapter 1: The Beginning",
            "Directed by CapBD",
        ];

        for text in text_presets {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(text).color(Color32::WHITE));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("+ Add").clicked() {
                            let start = timeline.playhead_frame;
                            let clip = Clip::new_text(text, start, 90);
                            timeline.add_clip_to_best_track(clip);
                        }
                    });
                });
            });
        }
    }

    fn render_audio_tab(ui: &mut Ui, timeline: &mut Timeline) {
        ui.label(RichText::new("Sound Effects & Audio").color(CapBdTheme::TEXT_SECONDARY).size(11.0));

        let audio_presets = [
            ("Background Beat (Synth)", 300),
            ("Whoosh Transition", 45),
            ("Film Beep Pulse", 30),
        ];

        for (name, duration) in audio_presets {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(name).color(Color32::WHITE));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("+ Add").clicked() {
                            let start = timeline.playhead_frame;
                            let clip = Clip::new_synthetic(name, MediaKind::Audio, TestPattern::AudioWaveformPulse, start, duration);
                            timeline.add_clip_to_best_track(clip);
                        }
                    });
                });
            });
        }
    }

    fn render_filters_tab(ui: &mut Ui, timeline: &mut Timeline) {
        ui.label(RichText::new("Color LUTs & Adjustment Presets").color(CapBdTheme::TEXT_SECONDARY).size(11.0));

        let filters = [
            ("Cyberpunk (High Sat + Vignette)", 1.4, 0.2, 0.4),
            ("Moody Film (Contrast + Warm)", 1.2, 0.3, 0.2),
            ("Black & White Monochrome", 0.0, 0.0, 0.3),
        ];

        for (name, sat, temp, vig) in filters {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new(name).color(Color32::WHITE));
                    if ui.button("Apply to Selected Clip").clicked() {
                        if let Some(sel_id) = timeline.selected_clip_id {
                            if let Some(clip) = timeline.find_clip_mut(sel_id) {
                                clip.color_grading.saturation = sat;
                                clip.color_grading.temperature = temp;
                                clip.color_grading.vignette = vig;
                            }
                        }
                    }
                });
            });
        }
    }
}
