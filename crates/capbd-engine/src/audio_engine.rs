use log::{info, warn};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Multi-track audio engine state for playback synchronization
pub struct AudioEngine {
    is_playing: Arc<AtomicBool>,
    master_volume: f32,
    is_muted: bool,
    has_audio_device: bool,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioEngine {
    pub fn new() -> Self {
        let has_audio_device = match rodio::OutputStream::try_default() {
            Ok((_stream, _handle)) => {
                info!("CapBD Audio Engine initialized with default output device");
                true
            }
            Err(e) => {
                warn!("No audio device available or permission denied (headless/CI): {}", e);
                false
            }
        };

        Self {
            is_playing: Arc::new(AtomicBool::new(false)),
            master_volume: 1.0,
            is_muted: false,
            has_audio_device,
        }
    }

    pub fn set_playing(&self, playing: bool) {
        self.is_playing.store(playing, Ordering::Relaxed);
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 2.0);
    }

    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    pub fn toggle_mute(&mut self) -> bool {
        self.is_muted = !self.is_muted;
        self.is_muted
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn has_device(&self) -> bool {
        self.has_audio_device
    }
}
