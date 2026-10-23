use crate::color::ColorGrading;
use crate::error::{CoreError, Result};
use crate::transform::Transform2D;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClipId(pub Uuid);

impl ClipId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ClipId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaKind {
    Video,
    Audio,
    Image,
    Text,
    Filter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestPattern {
    SmpteColorBars,
    NeonGradientMotion,
    CountdownLeader,
    CyberGrid,
    AudioWaveformPulse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaSource {
    Synthetic {
        pattern: TestPattern,
        label: String,
    },
    File {
        path: String,
        file_name: String,
    },
    Text {
        content: String,
        font_size: f32,
        font_color: [f32; 4],
    },
    ColorSolid {
        color: [f32; 4],
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    pub id: ClipId,
    pub name: String,
    pub kind: MediaKind,
    pub source: MediaSource,
    /// Absolute start position on timeline track in frames
    pub start_frame: u64,
    /// Duration of the clip on timeline in frames
    pub duration_frames: u64,
    /// Offset inside the source media (source in-point)
    pub in_point_frames: u64,
    /// Playback speed multiplier (1.0 = normal, 2.0 = 2x speed)
    pub speed: f32,
    /// Audio volume (1.0 = 100%, 0.0 = muted)
    pub volume: f32,
    /// Transform properties (Position, scale, rotation, opacity)
    pub transform: Transform2D,
    /// Color grading & filters
    pub color_grading: ColorGrading,
}

impl Clip {
    pub fn new_synthetic(
        name: impl Into<String>,
        kind: MediaKind,
        pattern: TestPattern,
        start_frame: u64,
        duration_frames: u64,
    ) -> Self {
        let name_str = name.into();
        Self {
            id: ClipId::new(),
            name: name_str.clone(),
            kind,
            source: MediaSource::Synthetic {
                pattern,
                label: name_str,
            },
            start_frame,
            duration_frames,
            in_point_frames: 0,
            speed: 1.0,
            volume: 1.0,
            transform: Transform2D::default(),
            color_grading: ColorGrading::default(),
        }
    }

    pub fn new_file(
        name: impl Into<String>,
        kind: MediaKind,
        path: impl Into<String>,
        start_frame: u64,
        duration_frames: u64,
    ) -> Self {
        let path_str = path.into();
        let name_str = name.into();
        Self {
            id: ClipId::new(),
            name: name_str,
            kind,
            source: MediaSource::File {
                path: path_str.clone(),
                file_name: path_str,
            },
            start_frame,
            duration_frames,
            in_point_frames: 0,
            speed: 1.0,
            volume: 1.0,
            transform: Transform2D::default(),
            color_grading: ColorGrading::default(),
        }
    }

    pub fn new_text(
        content: impl Into<String>,
        start_frame: u64,
        duration_frames: u64,
    ) -> Self {
        let text_content = content.into();
        Self {
            id: ClipId::new(),
            name: format!("Text: {}", text_content),
            kind: MediaKind::Text,
            source: MediaSource::Text {
                content: text_content,
                font_size: 48.0,
                font_color: [1.0, 1.0, 1.0, 1.0],
            },
            start_frame,
            duration_frames,
            in_point_frames: 0,
            speed: 1.0,
            volume: 1.0,
            transform: Transform2D::default(),
            color_grading: ColorGrading::default(),
        }
    }

    #[inline]
    pub fn end_frame(&self) -> u64 {
        self.start_frame + self.duration_frames
    }

    #[inline]
    pub fn contains_frame(&self, frame: u64) -> bool {
        frame >= self.start_frame && frame < self.end_frame()
    }

    /// Split this clip at the given timeline frame. Returns (left_clip, right_clip).
    pub fn split_at(&self, split_frame: u64) -> Result<(Clip, Clip)> {
        if split_frame <= self.start_frame || split_frame >= self.end_frame() {
            return Err(CoreError::InvalidSplitPoint {
                split_frame,
                start: self.start_frame,
                end: self.end_frame(),
            });
        }

        let left_duration = split_frame - self.start_frame;
        let right_duration = self.duration_frames - left_duration;

        let mut left = self.clone();
        left.duration_frames = left_duration;

        let mut right = self.clone();
        right.id = ClipId::new();
        right.start_frame = split_frame;
        right.duration_frames = right_duration;
        right.in_point_frames = self.in_point_frames + (left_duration as f32 * self.speed).round() as u64;

        Ok((left, right))
    }

    /// Trim start of the clip (move in-point forward or backward)
    pub fn trim_start(&mut self, new_start_frame: u64) -> Result<()> {
        let current_end = self.end_frame();
        if new_start_frame >= current_end {
            return Err(CoreError::InvalidDuration("New start must precede current end".into()));
        }
        let delta = new_start_frame as i64 - self.start_frame as i64;
        self.start_frame = new_start_frame;
        self.duration_frames = current_end - new_start_frame;
        self.in_point_frames = (self.in_point_frames as i64 + delta).max(0) as u64;
        Ok(())
    }

    /// Trim end of the clip (adjust duration)
    pub fn trim_end(&mut self, new_end_frame: u64) -> Result<()> {
        if new_end_frame <= self.start_frame {
            return Err(CoreError::InvalidDuration("New end must succeed current start".into()));
        }
        self.duration_frames = new_end_frame - self.start_frame;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_split() {
        let clip = Clip::new_synthetic(
            "Test Clip",
            MediaKind::Video,
            TestPattern::SmpteColorBars,
            100,
            200,
        );
        assert_eq!(clip.end_frame(), 300);

        let (left, right) = clip.split_at(150).expect("split should succeed");
        assert_eq!(left.start_frame, 100);
        assert_eq!(left.duration_frames, 50);
        assert_eq!(left.end_frame(), 150);

        assert_eq!(right.start_frame, 150);
        assert_eq!(right.duration_frames, 150);
        assert_eq!(right.end_frame(), 300);
        assert_ne!(left.id, right.id);
    }
}
