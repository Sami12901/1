use crate::theme::CapBdTheme;
use capbd_core::timeline::Timeline;
use egui::{Color32, RichText, ScrollArea, Ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorTab {
    Transform,
    Color,
    Speed,
    Audio,
}

pub struct InspectorPanel {
    pub active_tab: InspectorTab,
}

impl Default for InspectorPanel {
    fn default() -> Self {
        Self {
            active_tab: InspectorTab::Transform,
        }
    }
}

impl InspectorPanel {
    pub fn render(&mut self, ui: &mut Ui, timeline: &mut Timeline) {
        ui.vertical(|ui| {
            if let Some(sel_id) = timeline.selected_clip_id {
                let mut clip_opt = None;
                for track in &mut timeline.tracks {
                    if let Some(clip) = track.find_clip_mut(sel_id) {
                        clip_opt = Some(clip);
                        break;
                    }
                }

                if let Some(clip) = clip_opt {
                    // Header with clip name & kind
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(&clip.name)
                                .strong()
                                .color(Color32::WHITE)
                                .size(13.0),
                        );
                    });

                    ui.label(
                        RichText::new(format!(
                            "{:?} | In: {} | Duration: {} frames",
                            clip.kind, clip.start_frame, clip.duration_frames
                        ))
                        .size(11.0)
                        .color(CapBdTheme::TEXT_MUTED),
                    );

                    ui.separator();

                    // Inspector Tabs
                    ui.horizontal(|ui| {
                        ui.selectable_value(
                            &mut self.active_tab,
                            InspectorTab::Transform,
                            "Transform",
                        );
                        ui.selectable_value(&mut self.active_tab, InspectorTab::Color, "Color");
                        ui.selectable_value(&mut self.active_tab, InspectorTab::Speed, "Speed");
                        ui.selectable_value(&mut self.active_tab, InspectorTab::Audio, "Audio");
                    });

                    ui.separator();

                    ScrollArea::vertical().show(ui, |ui| match self.active_tab {
                        InspectorTab::Transform => {
                            ui.label(
                                RichText::new("Scale & Position")
                                    .strong()
                                    .color(CapBdTheme::TEXT_SECONDARY),
                            );

                            let mut scale_pct = clip.transform.scale_x * 100.0;
                            ui.horizontal(|ui| {
                                ui.label("Scale:");
                                if ui
                                    .add(
                                        egui::Slider::new(&mut scale_pct, 10.0..=500.0).suffix("%"),
                                    )
                                    .changed()
                                {
                                    clip.transform.set_scale_uniform(scale_pct / 100.0);
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Position X:");
                                ui.add(
                                    egui::DragValue::new(&mut clip.transform.position_x).speed(1.0),
                                );
                                ui.label("Y:");
                                ui.add(
                                    egui::DragValue::new(&mut clip.transform.position_y).speed(1.0),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Rotation:");
                                ui.add(
                                    egui::Slider::new(
                                        &mut clip.transform.rotation_deg,
                                        -180.0..=180.0,
                                    )
                                    .suffix("°"),
                                );
                            });

                            let mut opacity_pct = clip.transform.opacity * 100.0;
                            ui.horizontal(|ui| {
                                ui.label("Opacity:");
                                if ui
                                    .add(
                                        egui::Slider::new(&mut opacity_pct, 0.0..=100.0)
                                            .suffix("%"),
                                    )
                                    .changed()
                                {
                                    clip.transform.opacity = opacity_pct / 100.0;
                                }
                            });

                            if ui.button("Reset Transform").clicked() {
                                clip.transform.reset();
                            }
                        }
                        InspectorTab::Color => {
                            ui.label(
                                RichText::new("Color Adjustments")
                                    .strong()
                                    .color(CapBdTheme::TEXT_SECONDARY),
                            );

                            ui.horizontal(|ui| {
                                ui.label("Exposure:");
                                ui.add(
                                    egui::Slider::new(&mut clip.color_grading.exposure, -2.0..=2.0)
                                        .step_by(0.05),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Brightness:");
                                ui.add(
                                    egui::Slider::new(
                                        &mut clip.color_grading.brightness,
                                        -1.0..=1.0,
                                    )
                                    .step_by(0.05),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Contrast:");
                                ui.add(
                                    egui::Slider::new(&mut clip.color_grading.contrast, 0.0..=2.0)
                                        .step_by(0.05),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Saturation:");
                                ui.add(
                                    egui::Slider::new(
                                        &mut clip.color_grading.saturation,
                                        0.0..=2.0,
                                    )
                                    .step_by(0.05),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Temperature:");
                                ui.add(
                                    egui::Slider::new(
                                        &mut clip.color_grading.temperature,
                                        -1.0..=1.0,
                                    )
                                    .step_by(0.05),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Tint:");
                                ui.add(
                                    egui::Slider::new(&mut clip.color_grading.tint, -1.0..=1.0)
                                        .step_by(0.05),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("Vignette:");
                                ui.add(
                                    egui::Slider::new(&mut clip.color_grading.vignette, 0.0..=1.0)
                                        .step_by(0.05),
                                );
                            });

                            if ui.button("Reset Color").clicked() {
                                clip.color_grading.reset();
                            }
                        }
                        InspectorTab::Speed => {
                            ui.label(
                                RichText::new("Playback Speed")
                                    .strong()
                                    .color(CapBdTheme::TEXT_SECONDARY),
                            );
                            let speeds = [0.25, 0.5, 1.0, 1.5, 2.0, 4.0];
                            for s in speeds {
                                if ui
                                    .selectable_label(
                                        (clip.speed - s).abs() < 0.01,
                                        format!("{}x", s),
                                    )
                                    .clicked()
                                {
                                    clip.speed = s;
                                }
                            }
                        }
                        InspectorTab::Audio => {
                            ui.label(
                                RichText::new("Clip Audio")
                                    .strong()
                                    .color(CapBdTheme::TEXT_SECONDARY),
                            );
                            let mut vol_pct = clip.volume * 100.0;
                            if ui
                                .add(egui::Slider::new(&mut vol_pct, 0.0..=200.0).suffix("%"))
                                .changed()
                            {
                                clip.volume = vol_pct / 100.0;
                            }
                        }
                    });
                }
            } else {
                // No clip selected: show Project Overview & Shortcuts
                ui.label(
                    RichText::new("Project Properties")
                        .strong()
                        .color(Color32::WHITE),
                );
                ui.add_space(8.0);
                ui.label(format!("Aspect Ratio: {}", timeline.aspect_ratio.label()));
                ui.label(format!("Frame Rate: {:.1} FPS", timeline.fps));
                ui.label(format!("Total Tracks: {}", timeline.tracks.len()));
                ui.label(format!(
                    "Duration: {}",
                    timeline.total_timecode().to_smpte()
                ));

                ui.separator();
                ui.label(
                    RichText::new("Shortcuts")
                        .strong()
                        .color(CapBdTheme::TEXT_SECONDARY),
                );
                ui.label("Space : Play / Pause");
                ui.label("Ctrl+B : Split Clip");
                ui.label("Del : Delete Selected Clip");
                ui.label("Ctrl+Z : Undo");
                ui.label("Ctrl+Y : Redo");
                ui.label("Left/Right : Step 1 Frame");
                ui.label("Home/End : Jump Start/End");
            }
        });
    }
}
