use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreError {
    #[error("Clip not found: {0}")]
    ClipNotFound(String),

    #[error("Track not found: {0}")]
    TrackNotFound(String),

    #[error("Cannot split clip at frame {split_frame}: clip spans [{start}, {end})")]
    InvalidSplitPoint {
        split_frame: u64,
        start: u64,
        end: u64,
    },

    #[error("Invalid duration: {0}")]
    InvalidDuration(String),

    #[error("Project serialization failed: {0}")]
    SerializationError(String),

    #[error("Project deserialization failed: {0}")]
    DeserializationError(String),

    #[error("Undo stack is empty")]
    NothingToUndo,

    #[error("Redo stack is empty")]
    NothingToRedo,
}

pub type Result<T> = std::result::Result<T, CoreError>;
