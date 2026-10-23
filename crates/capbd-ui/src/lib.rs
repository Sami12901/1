pub mod export_dialog;
pub mod icons;
pub mod inspector_panel;
pub mod media_panel;
pub mod preview_player;
pub mod theme;
pub mod timeline_view;
pub mod top_bar;

pub use export_dialog::ExportDialog;
pub use icons::Icons;
pub use inspector_panel::InspectorPanel;
pub use media_panel::MediaPanel;
pub use preview_player::PreviewPlayer;
pub use theme::CapBdTheme;
pub use timeline_view::TimelineView;
pub use top_bar::{TopBar, TopBarAction};

use capbd_core::history::HistoryManager;
use capbd_core::project::Project;
use capbd_engine::audio_engine::AudioEngine;
use capbd_engine::video_decoder::MediaDecoderCache;
use egui::{vec2, CentralPanel, Color32, Key, Modifiers, SidePanel, TopBottomPanel};

pub struct CapBdApp {
    pub project: Project,
    pub history: HistoryManager,
    pub cache: MediaDecoderCache,
    pub audio_engine: AudioEngine,

    // Panels & Modals
    pub media_panel: MediaPanel,
    pub player: PreviewPlayer,
    pub timeline_view: TimelineView,
    pub inspector: InspectorPanel,
    pub export_dialog: ExportDialog,

    pub show_export_modal: bool,
    pub last_update_time: f64,
}

impl Default for CapBdApp {
    fn default() -> Self {
        Self::new()
    }
}

impl CapBdApp {
    pub fn new() -> Self {
        let mut app = Self {
            project: Project::new_default(),
            history: HistoryManager::default(),
            cache: MediaDecoderCache::new(),
            audio_engine: AudioEngine::new(),

            media_panel: MediaPanel::default(),
            player: PreviewPlayer::default(),
            timeline_view: TimelineView::default(),
            inspector: InspectorPanel::default(),
            export_dialog: ExportDialog::default(),

            show_export_modal: false,
            last_update_time: 0.0,
        };

        // Populate initial demo clips so the user can immediately experience CapBD
        app.setup_demo_project();
        app
    }

    fn setup_demo_project(&mut self) {
        use capbd_core::clip::{Clip, MediaKind, TestPattern};

        // 1. Video Clip 1: Neon Gradient (Intro)
        let clip1 = Clip::new_synthetic(
            "Intro Neon Sweep",
            MediaKind::Video,
            TestPattern::NeonGradientMotion,
            0,
            120, // 4s
        );
        self.project.timeline.add_clip_to_best_track(clip1);

        // 2. Video Clip 2: Film Countdown Leader
        let clip2 = Clip::new_synthetic(
            "Countdown Leader",
            MediaKind::Video,
            TestPattern::CountdownLeader,
            120,
            180, // 6s
        );
        self.project.timeline.add_clip_to_best_track(clip2);

        // 3. Video Clip 3: Cyber Grid (Outro)
        let clip3 = Clip::new_synthetic(
            "Cyber Grid Finale",
            MediaKind::Video,
            TestPattern::CyberGrid,
            300,
            150, // 5s
        );
        self.project.timeline.add_clip_to_best_track(clip3);

        // 4. Text Title Overlay
        let text_clip = Clip::new_text("CapBD: Photo & Video Editor", 30, 150);
        self.project.timeline.add_clip_to_best_track(text_clip);

        // 5. Audio Track
        let audio_clip = Clip::new_synthetic(
            "Master Audio BGM",
            MediaKind::Audio,
            TestPattern::AudioWaveformPulse,
            0,
            450, // 15s
        );
        self.project.timeline.add_clip_to_best_track(audio_clip);
    }

    /// Primary UI render loop
    pub fn update_ui(&mut self, ctx: &egui::Context) {
        CapBdTheme::apply(ctx);

        // 1. Handle Global Keyboard Shortcuts
        self.handle_shortcuts(ctx);

        // 2. Advance playhead when playing
        if self.audio_engine.is_playing() {
            let current_time = ctx.input(|i| i.time);
            let dt = (current_time - self.last_update_time).max(0.0);
            if dt > 0.0 {
                let frames_to_advance = (dt * self.project.timeline.fps).round() as u64;
                if frames_to_advance > 0 {
                    let total = self.project.timeline.total_duration_frames().max(1);
                    self.project.timeline.playhead_frame = (self.project.timeline.playhead_frame + frames_to_advance) % total;
                    self.last_update_time = current_time;
                }
            }
            ctx.request_repaint();
        } else {
            self.last_update_time = ctx.input(|i| i.time);
        }

        // 3. Top Navigation Bar
        TopBottomPanel::top("capbd_top_bar")
            .exact_height(48.0)
            .show(ctx, |ui| {
                let action = TopBar::render(
                    ui,
                    &mut self.project,
                    &self.history,
                    &mut self.show_export_modal,
                );

                match action {
                    TopBarAction::Undo => {
                        if let Ok(prev) = self.history.undo(self.project.timeline.clone()) {
                            self.project.timeline = prev;
                        }
                    }
                    TopBarAction::Redo => {
                        if let Ok(next) = self.history.redo(self.project.timeline.clone()) {
                            self.project.timeline = next;
                        }
                    }
                    TopBarAction::TakeSnapshot => {
                        let path = std::path::PathBuf::from("capbd_snapshot.png");
                        let _ = capbd_engine::exporter::TimelineCompositor::export_snapshot(
                            &self.project.timeline,
                            self.project.timeline.playhead_frame,
                            &path,
                            &self.cache,
                        );
                    }
                    TopBarAction::OpenExportModal => {
                        self.show_export_modal = true;
                    }
                    TopBarAction::None => {}
                }
            });

        // 4. Bottom Multi-Track Magnetic Timeline Panel
        TopBottomPanel::bottom("capbd_bottom_timeline")
            .exact_height(240.0)
            .show(ctx, |ui| {
                self.timeline_view.render(ui, &mut self.project.timeline, &mut self.history);
            });

        // 5. Left Media Asset Browser Panel
        SidePanel::left("capbd_left_media")
            .exact_width(260.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.media_panel.render(ui, &mut self.project.timeline);
            });

        // 6. Right Properties Inspector Panel
        SidePanel::right("capbd_right_inspector")
            .exact_width(280.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.inspector.render(ui, &mut self.project.timeline);
            });

        // 7. Central Video Preview Monitor
        CentralPanel::default().show(ctx, |ui| {
            self.player.render(
                ui,
                &mut self.project.timeline,
                &mut self.audio_engine,
                &self.cache,
            );
        });

        // 8. Export Modal Dialog
        if self.show_export_modal {
            self.export_dialog.render(
                ctx,
                &self.project.timeline,
                &self.cache,
                &mut self.show_export_modal,
            );
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Space: Play / Pause
            if i.key_pressed(Key::Space) {
                let playing = self.audio_engine.is_playing();
                self.audio_engine.set_playing(!playing);
            }

            // Ctrl+B: Split clip at playhead
            if i.modifiers.command && i.key_pressed(Key::B) {
                self.history.push_state(self.project.timeline.clone());
                let _ = self.project.timeline.split_clip_at_playhead();
            }

            // Del / Backspace: Delete selected clip
            if i.key_pressed(Key::Delete) || i.key_pressed(Key::Backspace) {
                if self.project.timeline.selected_clip_id.is_some() {
                    self.history.push_state(self.project.timeline.clone());
                    self.project.timeline.delete_selected_clip();
                }
            }

            // Ctrl+Z: Undo
            if i.modifiers.command && !i.modifiers.shift && i.key_pressed(Key::Z) {
                if let Ok(prev) = self.history.undo(self.project.timeline.clone()) {
                    self.project.timeline = prev;
                }
            }

            // Ctrl+Y or Ctrl+Shift+Z: Redo
            if (i.modifiers.command && i.key_pressed(Key::Y))
                || (i.modifiers.command && i.modifiers.shift && i.key_pressed(Key::Z))
            {
                if let Ok(next) = self.history.redo(self.project.timeline.clone()) {
                    self.project.timeline = next;
                }
            }

            // Left Arrow: Step back 1 frame
            if i.key_pressed(Key::ArrowLeft) {
                self.project.timeline.playhead_frame = self.project.timeline.playhead_frame.saturating_sub(1);
            }

            // Right Arrow: Step forward 1 frame
            if i.key_pressed(Key::ArrowRight) {
                let max_f = self.project.timeline.total_duration_frames().max(1);
                self.project.timeline.playhead_frame = (self.project.timeline.playhead_frame + 1).min(max_f);
            }

            // Home: Jump to start
            if i.key_pressed(Key::Home) {
                self.project.timeline.playhead_frame = 0;
            }

            // End: Jump to end
            if i.key_pressed(Key::End) {
                self.project.timeline.playhead_frame = self.project.timeline.total_duration_frames();
            }
        });
    }
}
