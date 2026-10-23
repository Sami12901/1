use serde::{Deserialize, Serialize};

/// 2D Transform for a video, image, or text clip
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform2D {
    /// Normalized or pixel position offset (X)
    pub position_x: f32,
    /// Normalized or pixel position offset (Y)
    pub position_y: f32,
    /// Scale factor along X (1.0 = 100%)
    pub scale_x: f32,
    /// Scale factor along Y (1.0 = 100%)
    pub scale_y: f32,
    /// Whether scale is linked uniformly
    pub uniform_scale: bool,
    /// Rotation in degrees (-180.0 to 180.0)
    pub rotation_deg: f32,
    /// Anchor point normalized (0.5, 0.5 is center)
    pub anchor_x: f32,
    pub anchor_y: f32,
    /// Opacity (0.0 transparent to 1.0 fully opaque)
    pub opacity: f32,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            uniform_scale: true,
            rotation_deg: 0.0,
            anchor_x: 0.5,
            anchor_y: 0.5,
            opacity: 1.0,
        }
    }
}

impl Transform2D {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn set_scale_uniform(&mut self, scale: f32) {
        let clamped = scale.clamp(0.01, 10.0);
        self.scale_x = clamped;
        self.scale_y = clamped;
    }
}
