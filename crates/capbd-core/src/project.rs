use crate::error::{CoreError, Result};
use crate::timeline::Timeline;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: String,
    pub version: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            name: "Untitled CapBD Project".into(),
            created_at: now,
            updated_at: now,
            author: "CapBD User".into(),
            version: "1.0.0".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub metadata: ProjectMetadata,
    pub timeline: Timeline,
}

impl Default for Project {
    fn default() -> Self {
        Self::new_default()
    }
}

impl Project {
    pub fn new_default() -> Self {
        Self {
            metadata: ProjectMetadata::default(),
            timeline: Timeline::new_default(),
        }
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| CoreError::SerializationError(e.to_string()))
    }

    pub fn from_json(json_str: &str) -> Result<Self> {
        serde_json::from_str(json_str)
            .map_err(|e| CoreError::DeserializationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clip::{Clip, MediaKind, TestPattern};

    #[test]
    fn test_project_json_roundtrip() {
        let mut proj = Project::new_default();
        proj.metadata.name = "My CapBD Masterpiece".into();
        let clip = Clip::new_synthetic(
            "Intro Title",
            MediaKind::Video,
            TestPattern::SmpteColorBars,
            0,
            120,
        );
        proj.timeline.add_clip_to_best_track(clip);

        let json = proj.to_json().expect("Serialization must succeed");
        let restored = Project::from_json(&json).expect("Deserialization must succeed");

        assert_eq!(restored.metadata.name, "My CapBD Masterpiece");
        assert_eq!(restored.timeline.total_duration_frames(), 120);
    }
}
