use serde::{Deserialize, Serialize};
use std::fmt;

/// Timecode representing an exact point or duration in video frames at a specific FPS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timecode {
    pub frames: u64,
    /// Store fps as scaled integer (e.g. 30_000 for 30.000, 29_970 for 29.970) for exact serialization
    pub fps_scaled: u32,
}

impl Timecode {
    pub const DEFAULT_FPS: f64 = 30.0;
    pub const DEFAULT_FPS_SCALED: u32 = 30_000;

    pub fn new(frames: u64, fps: f64) -> Self {
        Self {
            frames,
            fps_scaled: (fps * 1000.0).round() as u32,
        }
    }

    pub fn from_frames(frames: u64) -> Self {
        Self {
            frames,
            fps_scaled: Self::DEFAULT_FPS_SCALED,
        }
    }

    pub fn from_seconds(seconds: f64, fps: f64) -> Self {
        let frames = (seconds.max(0.0) * fps).round() as u64;
        Self::new(frames, fps)
    }

    #[inline]
    pub fn fps(&self) -> f64 {
        self.fps_scaled as f64 / 1000.0
    }

    #[inline]
    pub fn to_seconds(&self) -> f64 {
        let fps = self.fps();
        if fps <= 0.0 {
            0.0
        } else {
            self.frames as f64 / fps
        }
    }

    /// Converts timecode into standard SMPTE `HH:MM:SS:FF` representation.
    pub fn to_smpte(&self) -> String {
        let fps_val = self.fps().round() as u64;
        let fps_val = if fps_val == 0 { 30 } else { fps_val };

        let total_seconds = self.frames / fps_val;
        let remaining_frames = self.frames % fps_val;

        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        format!(
            "{:02}:{:02}:{:02}:{:02}",
            hours, minutes, seconds, remaining_frames
        )
    }

    /// Converts to short display format `MM:SS` or `MM:SS.f`
    pub fn to_short_time(&self) -> String {
        let total_seconds = self.to_seconds();
        let minutes = (total_seconds / 60.0).floor() as u64;
        let seconds = total_seconds % 60.0;
        format!("{:02}:{:04.1}", minutes, seconds)
    }
}

impl Default for Timecode {
    fn default() -> Self {
        Self::from_frames(0)
    }
}

impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_smpte())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timecode_conversions() {
        let tc = Timecode::new(90, 30.0);
        assert_eq!(tc.to_seconds(), 3.0);
        assert_eq!(tc.to_smpte(), "00:00:03:00");

        let tc2 = Timecode::from_seconds(65.5, 30.0);
        assert_eq!(tc2.to_smpte(), "00:01:05:15");
    }
}
