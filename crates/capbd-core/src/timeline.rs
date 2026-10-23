use crate::clip::{Clip, ClipId, MediaKind};
use crate::error::Result;
use crate::time::Timecode;
use crate::track::{Track, TrackId, TrackType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectRatio {
    Ratio16x9,
    Ratio9x16,
    Ratio1x1,
    Ratio4x5,
    Ratio21x9,
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self::Ratio16x9
    }
}

impl AspectRatio {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            AspectRatio::Ratio16x9 => (1920, 1080),
            AspectRatio::Ratio9x16 => (1080, 1920),
            AspectRatio::Ratio1x1 => (1080, 1080),
            AspectRatio::Ratio4x5 => (1080, 1350),
            AspectRatio::Ratio21x9 => (2560, 1080),
        }
    }

    pub fn aspect_f32(&self) -> f32 {
        let (w, h) = self.dimensions();
        w as f32 / h as f32
    }

    pub fn label(&self) -> &'static str {
        match self {
            AspectRatio::Ratio16x9 => "16:9 (Landscape)",
            AspectRatio::Ratio9x16 => "9:16 (TikTok / Reels)",
            AspectRatio::Ratio1x1 => "1:1 (Square)",
            AspectRatio::Ratio4x5 => "4:5 (Portrait)",
            AspectRatio::Ratio21x9 => "21:9 (Ultrawide)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            AspectRatio::Ratio16x9 => "16:9",
            AspectRatio::Ratio9x16 => "9:16",
            AspectRatio::Ratio1x1 => "1:1",
            AspectRatio::Ratio4x5 => "4:5",
            AspectRatio::Ratio21x9 => "21:9",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timeline {
    pub tracks: Vec<Track>,
    pub playhead_frame: u64,
    pub fps: f64,
    pub selected_clip_id: Option<ClipId>,
    pub selected_track_id: Option<TrackId>,
    pub aspect_ratio: AspectRatio,
    pub snap_to_clips: bool,
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new_default()
    }
}

impl Timeline {
    pub fn new_default() -> Self {
        let mut tracks = Vec::new();

        // 1. Text Track
        tracks.push(Track::new("Text & Titles", TrackType::Text));

        // 2. Video Overlay / PIP Track
        tracks.push(Track::new("Overlay Video (PIP)", TrackType::Video));

        // 3. Main Storyline Video Track
        tracks.push(Track::new("Main Video", TrackType::Video));

        // 4. Background Music & Sound Effects Track
        tracks.push(Track::new("Audio & Music", TrackType::Audio));

        Self {
            tracks,
            playhead_frame: 0,
            fps: 30.0,
            selected_clip_id: None,
            selected_track_id: None,
            aspect_ratio: AspectRatio::Ratio16x9,
            snap_to_clips: true,
        }
    }

    pub fn total_duration_frames(&self) -> u64 {
        self.tracks
            .iter()
            .map(|t| t.duration_frames())
            .max()
            .unwrap_or(0)
    }

    pub fn current_timecode(&self) -> Timecode {
        Timecode::new(self.playhead_frame, self.fps)
    }

    pub fn total_timecode(&self) -> Timecode {
        Timecode::new(self.total_duration_frames(), self.fps)
    }

    pub fn find_clip(&self, id: ClipId) -> Option<&Clip> {
        for track in &self.tracks {
            if let Some(clip) = track.find_clip(id) {
                return Some(clip);
            }
        }
        None
    }

    pub fn find_clip_mut(&mut self, id: ClipId) -> Option<&mut Clip> {
        for track in &mut self.tracks {
            if let Some(clip) = track.find_clip_mut(id) {
                return Some(clip);
            }
        }
        None
    }

    pub fn add_clip_to_best_track(&mut self, clip: Clip) -> TrackId {
        let kind = clip.kind;
        let target_track = self
            .tracks
            .iter_mut()
            .find(|t| t.track_type.matches_media_kind(kind));

        if let Some(track) = target_track {
            let track_id = track.id;
            track.add_clip(clip);
            track_id
        } else {
            // Create a new track for it
            let new_type = match kind {
                MediaKind::Video | MediaKind::Image => TrackType::Video,
                MediaKind::Audio => TrackType::Audio,
                MediaKind::Text => TrackType::Text,
                MediaKind::Filter => TrackType::Effect,
            };
            let mut new_track = Track::new(format!("{:?} Track", kind), new_type);
            let track_id = new_track.id;
            new_track.add_clip(clip);
            self.tracks.push(new_track);
            track_id
        }
    }

    /// Split whatever clip is currently under the playhead on the selected track or on the main video track
    pub fn split_clip_at_playhead(&mut self) -> Result<Option<ClipId>> {
        let frame = self.playhead_frame;

        // Try selected clip first if valid
        if let Some(sel_id) = self.selected_clip_id {
            for track in &mut self.tracks {
                if let Some(clip) = track.find_clip(sel_id) {
                    if clip.contains_frame(frame) && frame > clip.start_frame {
                        let new_id = track.split_clip(sel_id, frame)?;
                        self.selected_clip_id = Some(new_id);
                        return Ok(Some(new_id));
                    }
                }
            }
        }

        // Otherwise find first clip at playhead
        for track in &mut self.tracks {
            if let Some(clip) = track.clip_at(frame) {
                if frame > clip.start_frame {
                    let clip_id = clip.id;
                    let new_id = track.split_clip(clip_id, frame)?;
                    self.selected_clip_id = Some(new_id);
                    return Ok(Some(new_id));
                }
            }
        }

        Ok(None)
    }

    /// Deletes the currently selected clip and clears selection
    pub fn delete_selected_clip(&mut self) -> Option<Clip> {
        let sel_id = self.selected_clip_id?;
        for track in &mut self.tracks {
            if let Some(removed) = track.remove_clip(sel_id) {
                self.selected_clip_id = None;
                return Some(removed);
            }
        }
        None
    }

    /// Snaps a candidate frame to nearby clip edges or playhead
    pub fn snap_frame(&self, target_frame: u64, threshold: u64) -> u64 {
        if !self.snap_to_clips {
            return target_frame;
        }

        let mut closest_dist = threshold + 1;
        let mut snapped = target_frame;

        // Check 0
        let dist_zero = target_frame;
        if dist_zero <= threshold && dist_zero < closest_dist {
            closest_dist = dist_zero;
            snapped = 0;
        }

        // Check playhead
        let dist_playhead = (target_frame as i64 - self.playhead_frame as i64).unsigned_abs();
        if dist_playhead <= threshold && dist_playhead < closest_dist {
            closest_dist = dist_playhead;
            snapped = self.playhead_frame;
        }

        // Check clip boundaries
        for track in &self.tracks {
            for clip in &track.clips {
                let d_start = (target_frame as i64 - clip.start_frame as i64).unsigned_abs();
                if d_start <= threshold && d_start < closest_dist {
                    closest_dist = d_start;
                    snapped = clip.start_frame;
                }

                let d_end = (target_frame as i64 - clip.end_frame() as i64).unsigned_abs();
                if d_end <= threshold && d_end < closest_dist {
                    closest_dist = d_end;
                    snapped = clip.end_frame();
                }
            }
        }

        snapped
    }

    /// Collect all clips active at the specified frame across all tracks, sorted from bottom to top for layering
    pub fn get_active_clips_at(&self, frame: u64) -> Vec<(&Track, &Clip)> {
        let mut active = Vec::new();
        // Traverse in reverse track order (main video bottom, overlay on top, text higher)
        for track in self.tracks.iter().rev() {
            if track.is_hidden {
                continue;
            }
            if let Some(clip) = track.clip_at(frame) {
                active.push((track, clip));
            }
        }
        active
    }
}
