use crate::clip::{Clip, ClipId, MediaKind};
use crate::error::{CoreError, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub Uuid);

impl TrackId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TrackId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TrackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackType {
    /// Video tracks (Main storyline, PIP overlays)
    Video,
    /// Audio tracks (BGM, Voiceover, SFX)
    Audio,
    /// Text and Titles
    Text,
    /// Adjustment layers & Filters
    Effect,
}

impl TrackType {
    pub fn default_name(&self, index: usize) -> String {
        match self {
            TrackType::Video => {
                if index == 0 {
                    "Main Video".into()
                } else {
                    format!("Overlay V{}", index + 1)
                }
            }
            TrackType::Audio => format!("Audio A{}", index + 1),
            TrackType::Text => format!("Text T{}", index + 1),
            TrackType::Effect => format!("Effects E{}", index + 1),
        }
    }

    pub fn matches_media_kind(&self, kind: MediaKind) -> bool {
        match (self, kind) {
            (TrackType::Video, MediaKind::Video | MediaKind::Image) => true,
            (TrackType::Audio, MediaKind::Audio) => true,
            (TrackType::Text, MediaKind::Text) => true,
            (TrackType::Effect, MediaKind::Filter) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub track_type: TrackType,
    pub clips: Vec<Clip>,
    pub is_muted: bool,
    pub is_locked: bool,
    pub is_hidden: bool,
}

impl Track {
    pub fn new(name: impl Into<String>, track_type: TrackType) -> Self {
        Self {
            id: TrackId::new(),
            name: name.into(),
            track_type,
            clips: Vec::new(),
            is_muted: false,
            is_locked: false,
            is_hidden: false,
        }
    }

    pub fn duration_frames(&self) -> u64 {
        self.clips.iter().map(|c| c.end_frame()).max().unwrap_or(0)
    }

    pub fn sort_clips(&mut self) {
        self.clips.sort_by_key(|c| c.start_frame);
    }

    pub fn add_clip(&mut self, clip: Clip) {
        self.clips.push(clip);
        self.sort_clips();
    }

    pub fn remove_clip(&mut self, id: ClipId) -> Option<Clip> {
        if let Some(pos) = self.clips.iter().position(|c| c.id == id) {
            Some(self.clips.remove(pos))
        } else {
            None
        }
    }

    pub fn find_clip(&self, id: ClipId) -> Option<&Clip> {
        self.clips.iter().find(|c| c.id == id)
    }

    pub fn find_clip_mut(&mut self, id: ClipId) -> Option<&mut Clip> {
        self.clips.iter_mut().find(|c| c.id == id)
    }

    pub fn clip_at(&self, frame: u64) -> Option<&Clip> {
        self.clips.iter().find(|c| c.contains_frame(frame))
    }

    pub fn clip_at_mut(&mut self, frame: u64) -> Option<&mut Clip> {
        self.clips.iter_mut().find(|c| c.contains_frame(frame))
    }

    /// Split clip with specified id at the given frame
    pub fn split_clip(&mut self, id: ClipId, frame: u64) -> Result<ClipId> {
        let index = self
            .clips
            .iter()
            .position(|c| c.id == id)
            .ok_or_else(|| CoreError::ClipNotFound(id.to_string()))?;

        let (left, right) = self.clips[index].split_at(frame)?;
        let right_id = right.id;
        self.clips[index] = left;
        self.clips.insert(index + 1, right);
        self.sort_clips();
        Ok(right_id)
    }
}
