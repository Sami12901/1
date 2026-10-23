pub mod clip;
pub mod color;
pub mod error;
pub mod history;
pub mod project;
pub mod time;
pub mod timeline;
pub mod track;
pub mod transform;

pub use clip::{Clip, ClipId, MediaKind, MediaSource, TestPattern};
pub use color::ColorGrading;
pub use error::{CoreError, Result};
pub use history::HistoryManager;
pub use project::{Project, ProjectMetadata};
pub use time::Timecode;
pub use timeline::{AspectRatio, Timeline};
pub use track::{Track, TrackId, TrackType};
pub use transform::Transform2D;
