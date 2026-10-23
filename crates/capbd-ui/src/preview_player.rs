use crate::icons::Icons;
use crate::theme::CapBdTheme;
use capbd_core::timeline::Timeline;
use capbd_engine::audio_engine::AudioEngine;
use capbd_engine::exporter::TimelineCompositor;
use capbd_engine::video_decoder::MediaDecoderCache;
use egui::{vec2, Color32, ColorImage, Rect, RichText, Sense, TextureHandle, TextureOptions, Ui, Vec2};

pub struct PreviewPlayer {
    texture: Option<TextureHandle>,
    last_rendered_frame: Option<u64>,
}

impl Default for PreviewPlayer {
    fn default() -> Self {
        Self {
            texture: None,
            last_rendered_frame: None,
        }
    }
}

impl PreviewPlayer {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        timeline: &mut Timeline,
        audio_engine: &mut AudioEngine,
        cache: &MediaDecoderCache,
    ) {
        ui.vertical(|ui| {
            // Viewport Canvas Area
            let avail_size = ui.available_size();
            let transport_bar_height = 44.0;
            let viewport_height = (avail_size.y - transport_bar_height).max(100.0);
            let viewport_size = vec2(avail_size.x, viewport_height);

            let (canvas_rect, _canvas_resp) = ui.allocate_exact_size(viewport_size, Sense::hover());
            let painter = ui.painter_at(canvas_rect);

            // Fill background with matte dark letterbox
            painter.rect_filled(canvas_rect, 6.0, Color32::from_rgb(10, 10, 14));

            // Compute aspect-ratio fitted video rectangle inside viewport
            let aspect_ratio = timeline.aspect_ratio.aspect_f32();
            let mut video_w = canvas_rect.width() - 16.0;
            let mut video_h = video_w / aspect_ratio;

            if video_h > (canvas_rect.height() - 16.0) {
                video_h = canvas_rect.height() - 16.0;
                video_w = video_h * aspect_ratio;
            }

            let center = canvas_rect.center();
            let video_rect = Rect::from_center_size(center, vec2(video_w, video_h));

            // Render current frame if frame changed or texture is missing
            let current_frame = timeline.playhead_frame;
            let needs_update = self.texture.is_none() || self.last_rendered_frame != Some(current_frame);

            if needs_update {
                // Render at 640x360 or proportional preview resolution for fluid performance
                let preview_h = 360;
                let preview_w = (preview_h as f32 * aspect_ratio).round() as u32;

                let fb = TimelineCompositor::render_frame(timeline, current_frame, preview_w, preview_h, cache);
                let color_image = ColorImage::from_rgba_unmultiplied(
                    [fb.width as usize, fb.height as usize],
                    &fb.data,
                );

                self.texture = Some(ui.ctx().load_texture(
                    "video_preview_texture",
                    color_image,
                    TextureOptions::LINEAR,
                ));
                self.last_rendered_frame = Some(current_frame);
            }

            if let Some(tex) = &self.texture {
                painter.image(
                    tex.id(),
                    video_rect,
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
            }

            // Outline around active video area
            painter.rect_stroke(video_rect, 0.0, egui::Stroke::new(1.0, Color32::from_rgb(34, 38, 48)));

            // Transport Control Bar
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = vec2(8.0, 0.0);

                // Jump to Start
                if Icons::button_custom(ui, "btn_jump_start", vec2(28.0, 28.0), "Jump to Start (Home)", |p, r, c| {
                    Icons::draw_step_back(p, r, c);
                })
                .clicked()
                {
                    timeline.playhead_frame = 0;
                }

                // Step Back 1 Frame
                if Icons::button_custom(ui, "btn_step_back", vec2(28.0, 28.0), "Step Back 1 Frame (Left Arrow)", |p, r, c| {
                    Icons::draw_step_back(p, r, c);
                })
                .clicked()
                {
                    timeline.playhead_frame = timeline.playhead_frame.saturating_sub(1);
                }

                // Play / Pause Button
                let is_playing = audio_engine.is_playing();
                let play_resp = Icons::button_custom(
                    ui,
                    "btn_play_pause",
                    vec2(34.0, 34.0),
                    if is_playing { "Pause (Space)" } else { "Play (Space)" },
                    |p, r, c| {
                        if is_playing {
                            Icons::draw_pause(p, r, c);
                        } else {
                            Icons::draw_play(p, r, c);
                        }
                    },
                );
                if play_resp.clicked() {
                    audio_engine.set_playing(!is_playing);
                }

                // Step Forward 1 Frame
                if Icons::button_custom(ui, "btn_step_fwd", vec2(28.0, 28.0), "Step Forward 1 Frame (Right Arrow)", |p, r, c| {
                    Icons::draw_step_forward(p, r, c);
                })
                .clicked()
                {
                    timeline.playhead_frame = (timeline.playhead_frame + 1).min(timeline.total_duration_frames().max(1));
                }

                // Jump to End
                if Icons::button_custom(ui, "btn_jump_end", vec2(28.0, 28.0), "Jump to End (End)", |p, r, c| {
                    Icons::draw_step_forward(p, r, c);
                })
                .clicked()
                {
                    timeline.playhead_frame = timeline.total_duration_frames();
                }

                ui.separator();

                // Timecode Display: Current / Total
                let cur_tc = timeline.current_timecode().to_smpte();
                let tot_tc = timeline.total_timecode().to_smpte();
                ui.label(
                    RichText::new(format!("{} / {}", cur_tc, tot_tc))
                        .color(CapBdTheme::TEXT_PRIMARY)
                        .strong()
                        .monospace()
                        .size(13.0),
                );

                // Right aligned audio controls
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut vol = audio_engine.master_volume();
                    if ui.add(egui::Slider::new(&mut vol, 0.0..=1.5).show_value(false)).changed() {
                        audio_engine.set_master_volume(vol);
                    }

                    let muted = audio_engine.is_muted();
                    if Icons::button_custom(ui, "btn_mute_vol", vec2(28.0, 28.0), "Mute Audio", |p, r, _| {
                        Icons::draw_volume(p, r, muted);
                    })
                    .clicked()
                    {
                        audio_engine.toggle_mute();
                    }
                });
            });
        });
    }
}
