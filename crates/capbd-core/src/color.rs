use serde::{Deserialize, Serialize};

/// Color Grading & Filter Adjustments for video and photo clips (CapCut style)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorGrading {
    /// Exposure adjustment (-2.0 to +2.0, default 0.0)
    pub exposure: f32,
    /// Brightness adjustment (-1.0 to +1.0, default 0.0)
    pub brightness: f32,
    /// Contrast factor (0.0 to 2.0, default 1.0)
    pub contrast: f32,
    /// Saturation factor (0.0 grayscale to 2.0 hyper-saturated, default 1.0)
    pub saturation: f32,
    /// Color temperature (-1.0 blue/cool to +1.0 amber/warm, default 0.0)
    pub temperature: f32,
    /// Color tint (-1.0 green to +1.0 magenta, default 0.0)
    pub tint: f32,
    /// Vignette intensity (0.0 none to 1.0 strong dark edges, default 0.0)
    pub vignette: f32,
}

impl Default for ColorGrading {
    fn default() -> Self {
        Self {
            exposure: 0.0,
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            temperature: 0.0,
            tint: 0.0,
            vignette: 0.0,
        }
    }
}

impl ColorGrading {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn is_identity(&self) -> bool {
        self.exposure.abs() < 0.001
            && self.brightness.abs() < 0.001
            && (self.contrast - 1.0).abs() < 0.001
            && (self.saturation - 1.0).abs() < 0.001
            && self.temperature.abs() < 0.001
            && self.tint.abs() < 0.001
            && self.vignette.abs() < 0.001
    }
}
