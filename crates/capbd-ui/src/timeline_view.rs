use crate::icons::Icons;
use crate::theme::CapBdTheme;
use capbd_core::clip::{ClipId, MediaKind};
use capbd_core::history::HistoryManager;
use capbd_core::timeline::Timeline;
use egui::{
    pos2, vec2, Color32, CornerRadius, Pos2, Rect, Response, RichText, ScrollArea, Sense, Stroke,
    Ui, Vec2,
};

pub struct TimelineView {
    pub zoom_px_per_frame: f32,
    pub is_scrubbing: bool,
    pub dragging_trim: Option<(ClipId, bool)>, // (ClipId, is_start_handle)
}

impl Default for TimelineView {
    fn default() -> Self {
        Self {
            zoom_px_per_frame: 3.5, // 3.5 px per frame default (~105px per second)
            is_scrubbing: false,
            dragging_trim: None,
        }
    }
}

impl TimelineView {
    pub fn render(
        &mut self,
        ui: &mut Ui,
        timeline: &mut Timeline,
        history: &mut HistoryManager,
    ) {
        ui.vertical(|ui| {
            // 1. Timeline Toolbar
            self.render_toolbar(ui, timeline, history);

            ui.separator();

            // 2. Timeline Tracks & Ruler Canvas
            self.render_tracks_area(ui, timeline, history);
        });
    }

    fn render_toolbar(
        &mut self,
        ui: &mut Ui,
        timeline: &mut Timeline,
        history: &mut HistoryManager,
    ) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(8.0, 0.0);

            // Split Tool (Scissors)
            let split_btn = Icons::button_custom(
                ui,
                "btn_split",
                vec2(28.0, 28.0),
                "Split Clip at Playhead (Ctrl+B)",
                |p, r, c| {
                    Icons::draw_scissors(p, r, c);
                },
            );
            if split_btn.clicked() {
                history.push_state(timeline.clone());
                let _ = timeline.split_clip_at_playhead();
            }

            // Delete Clip Tool (Trash)
            let has_selection = timeline.selected_clip_id.is_some();
            let delete_btn = Icons::button_custom(
                ui,
                "btn_delete",
                vec2(28.0, 28.0),
                "Delete Selected Clip (Del)",
                |p, r, c| {
                    let col = if has_selection { c } else { CapBdTheme::TEXT_MUTED };
                    Icons::draw_trash(p, r, col);
                },
            );
            if delete_btn.clicked() && has_selection {
                history.push_state(timeline.clone());
                timeline.delete_selected_clip();
            }

            ui.separator();

            // Magnetic Snap Toggle
            let magnet_btn = Icons::button_custom(
                ui,
                "btn_magnet",
                vec2(28.0, 28.0),
                "Toggle Magnetic Snap",
                |p, r, _| {
                    Icons::draw_magnet(p, r, timeline.snap_to_clips);
                },
            );
            if magnet_btn.clicked() {
                timeline.snap_to_clips = !timeline.snap_to_clips;
            }

            // Timeline Zoom Slider
            ui.label(RichText::new("Zoom").color(CapBdTheme::TEXT_MUTED).size(11.0));
            ui.add(egui::Slider::new(&mut self.zoom_px_per_frame, 1.0..=15.0).show_value(false));

            // Current Frame / Total indicator
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!(
                        "Frame {} of {}",
                        timeline.playhead_frame,
                        timeline.total_duration_frames()
                    ))
                    .size(11.0)
                    .color(CapBdTheme::TEXT_MUTED),
                );
            });
        });
    }

    fn render_tracks_area(
        &mut self,
        ui: &mut Ui,
        timeline: &mut Timeline,
        history: &mut HistoryManager,
    ) {
        let header_width = 140.0;
        let ruler_height = 24.0;
        let track_height = 48.0;

        let total_frames = timeline.total_duration_frames().max(300);
        let content_width = (total_frames as f32 * self.zoom_px_per_frame).max(ui.available_width() - header_width);

        ScrollArea::both().show(ui, |ui| {
            let total_tracks = timeline.tracks.len();
            let total_height = ruler_height + (total_tracks as f32 * (track_height + 4.0)) + 40.0;

            let (full_rect, full_resp) = ui.allocate_exact_size(vec2(header_width + content_width, total_height), Sense::click_and_drag());
            let painter = ui.painter_at(full_rect);

            let tracks_rect = Rect::from_min_max(
                pos2(full_rect.left() + header_width, full_rect.top()),
                full_rect.max,
            );

            // Handle scrubbing clicks/drags in tracks area
            if full_resp.dragged() || full_resp.clicked() {
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    if mouse_pos.x >= tracks_rect.left() {
                        let rel_x = mouse_pos.x - tracks_rect.left();
                        let raw_frame = (rel_x / self.zoom_px_per_frame).round().max(0.0) as u64;
                        let snapped = timeline.snap_frame(raw_frame, 6);
                        timeline.playhead_frame = snapped;
                    }
                }
            }

            // 1. Draw Time Ruler at the top
            let ruler_rect = Rect::from_min_size(tracks_rect.min, vec2(tracks_rect.width(), ruler_height));
            painter.rect_filled(ruler_rect, 0.0, Color32::from_rgb(20, 20, 26));
            painter.line_segment([ruler_rect.left_bottom(), ruler_rect.right_bottom()], Stroke::new(1.0, CapBdTheme::BORDER));

            // Time ruler tick marks (every 30 frames / 1s)
            let frames_per_second = timeline.fps.round() as u64;
            for f in (0..=total_frames).step_by(frames_per_second as usize) {
                let x = tracks_rect.left() + (f as f32 * self.zoom_px_per_frame);
                let seconds = f / frames_per_second;
                let minutes = seconds / 60;
                let sec_rem = seconds % 60;

                // Major tick
                painter.line_segment([pos2(x, ruler_rect.bottom() - 10.0), pos2(x, ruler_rect.bottom())], Stroke::new(1.0, Color32::from_rgb(100, 100, 115)));

                painter.text(
                    pos2(x + 3.0, ruler_rect.top() + 4.0),
                    egui::Align2::LEFT_TOP,
                    format!("{:02}:{:02}", minutes, sec_rem),
                    egui::FontId::monospace(10.0),
                    CapBdTheme::TEXT_MUTED,
                );
            }

            // 2. Draw Track Lanes
            for (track_idx, track) in timeline.tracks.iter_mut().enumerate() {
                let track_top = ruler_rect.bottom() + 4.0 + (track_idx as f32 * (track_height + 4.0));
                let track_lane_rect = Rect::from_min_size(pos2(tracks_rect.left(), track_top), vec2(tracks_rect.width(), track_height));
                let track_header_rect = Rect::from_min_size(pos2(full_rect.left(), track_top), vec2(header_width, track_height));

                // Header
                painter.rect_filled(track_header_rect, 4.0, CapBdTheme::BG_CARD);
                painter.rect_stroke(track_header_rect, 4.0, Stroke::new(1.0, CapBdTheme::BORDER));

                painter.text(
                    pos2(track_header_rect.left() + 8.0, track_header_rect.top() + 8.0),
                    egui::Align2::LEFT_TOP,
                    &track.name,
                    egui::FontId::proportional(12.0),
                    Color32::WHITE,
                );

                // Track lane background
                painter.rect_filled(track_lane_rect, 4.0, Color32::from_rgb(18, 18, 23));
                painter.rect_stroke(track_lane_rect, 4.0, Stroke::new(1.0, Color32::from_rgb(28, 28, 36)));

                // Clips in track
                for clip in &track.clips {
                    let clip_x = tracks_rect.left() + (clip.start_frame as f32 * self.zoom_px_per_frame);
                    let clip_w = (clip.duration_frames as f32 * self.zoom_px_per_frame).max(8.0);
                    let clip_rect = Rect::from_min_size(pos2(clip_x, track_top + 4.0), vec2(clip_w, track_height - 8.0));

                    let is_selected = timeline.selected_clip_id == Some(clip.id);

                    let clip_color = match clip.kind {
                        MediaKind::Video => CapBdTheme::CLIP_VIDEO,
                        MediaKind::Audio => CapBdTheme::CLIP_AUDIO,
                        MediaKind::Text => CapBdTheme::CLIP_TEXT,
                        MediaKind::Filter => CapBdTheme::CLIP_EFFECT,
                        MediaKind::Image => Color32::from_rgb(0, 180, 216),
                    };

                    painter.rect_filled(clip_rect, CornerRadius::same(4), clip_color);

                    // Selection highlight
                    if is_selected {
                        painter.rect_stroke(
                            clip_rect,
                            CornerRadius::same(4),
                            Stroke::new(2.0, CapBdTheme::ACCENT_CYAN),
                        );
                    } else {
                        painter.rect_stroke(
                            clip_rect,
                            CornerRadius::same(4),
                            Stroke::new(1.0, Color32::from_white_alpha(40)),
                        );
                    }

                    // Trim Handles on Left and Right
                    let left_handle = Rect::from_min_size(clip_rect.min, vec2(6.0, clip_rect.height()));
                    let right_handle = Rect::from_min_size(pos2(clip_rect.right() - 6.0, clip_rect.top()), vec2(6.0, clip_rect.height()));
                    painter.rect_filled(left_handle, 2.0, Color32::from_black_alpha(80));
                    painter.rect_filled(right_handle, 2.0, Color32::from_black_alpha(80));

                    // Clip Title Text
                    painter.text(
                        pos2(clip_rect.left() + 10.0, clip_rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        &clip.name,
                        egui::FontId::proportional(11.0),
                        Color32::WHITE,
                    );

                    // Click detection on clip
                    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                        if clip_rect.contains(mouse_pos) && ui.input(|i| i.pointer.any_pressed()) {
                            timeline.selected_clip_id = Some(clip.id);
                        }
                    }
                }
            }

            // 3. Draw Red/Cyan Scrubber Playhead Line
            let playhead_x = tracks_rect.left() + (timeline.playhead_frame as f32 * self.zoom_px_per_frame);
            painter.line_segment(
                [pos2(playhead_x, ruler_rect.top()), pos2(playhead_x, full_rect.bottom())],
                Stroke::new(2.0, Color32::from_rgb(255, 59, 48)), // CapCut Playhead Red
            );

            // Playhead scrubber handle at top
            let handle_head = vec![
                pos2(playhead_x - 6.0, ruler_rect.top()),
                pos2(playhead_x + 6.0, ruler_rect.top()),
                pos2(playhead_x + 6.0, ruler_rect.top() + 10.0),
                pos2(playhead_x, ruler_rect.top() + 16.0),
                pos2(playhead_x - 6.0, ruler_rect.top() + 10.0),
            ];
            painter.add(egui::epaint::PathShape::convex_polygon(
                handle_head,
                Color32::from_rgb(255, 59, 48),
                Stroke::NONE,
            ));
        });
    }
}
