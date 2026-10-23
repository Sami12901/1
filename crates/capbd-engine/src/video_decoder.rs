use capbd_core::clip::{Clip, MediaSource, TestPattern};
use image::{Rgba, RgbaImage};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

/// Cached frame buffer (RGBA8 format)
#[derive(Debug, Clone)]
pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl FrameBuffer {
    pub fn new_solid(width: u32, height: u32, color: [u8; 4]) -> Self {
        let pixel_count = (width * height) as usize;
        let mut data = Vec::with_capacity(pixel_count * 4);
        for _ in 0..pixel_count {
            data.extend_from_slice(&color);
        }
        Self {
            width,
            height,
            data,
        }
    }

    pub fn new_empty(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            width,
            height,
            data: vec![0; size],
        }
    }
}

/// In-memory image and generator cache
pub struct MediaDecoderCache {
    image_cache: Mutex<HashMap<String, FrameBuffer>>,
}

impl Default for MediaDecoderCache {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaDecoderCache {
    pub fn new() -> Self {
        Self {
            image_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Render or retrieve a frame for a given clip at a specific timeline frame
    pub fn get_frame(
        &self,
        clip: &Clip,
        timeline_frame: u64,
        width: u32,
        height: u32,
    ) -> FrameBuffer {
        let relative_frame = timeline_frame.saturating_sub(clip.start_frame) + clip.in_point_frames;

        match &clip.source {
            MediaSource::Synthetic { pattern, .. } => {
                Self::generate_synthetic_pattern(*pattern, relative_frame, width, height)
            }
            MediaSource::ColorSolid { color } => {
                let r = (color[0] * 255.0) as u8;
                let g = (color[1] * 255.0) as u8;
                let b = (color[2] * 255.0) as u8;
                let a = (color[3] * 255.0) as u8;
                FrameBuffer::new_solid(width, height, [r, g, b, a])
            }
            MediaSource::File { path, .. } => self.load_file_frame(path, width, height),
            MediaSource::Text {
                content,
                font_size,
                font_color,
            } => Self::generate_text_placeholder(content, *font_size, *font_color, width, height),
        }
    }

    fn load_file_frame(&self, path_str: &str, target_w: u32, target_h: u32) -> FrameBuffer {
        let mut cache = self.image_cache.lock().unwrap();
        if let Some(cached) = cache.get(path_str) {
            return cached.clone();
        }

        let path = Path::new(path_str);
        if path.exists() {
            if let Ok(img) = image::open(path) {
                let rgba = img.to_rgba8();
                let fb = FrameBuffer {
                    width: rgba.width(),
                    height: rgba.height(),
                    data: rgba.into_raw(),
                };
                cache.insert(path_str.to_string(), fb.clone());
                return fb;
            }
        }

        // Fallback test frame if file cannot be read
        Self::generate_synthetic_pattern(TestPattern::NeonGradientMotion, 0, target_w, target_h)
    }

    /// Generates high-fidelity procedural patterns (real-time generated video frames)
    pub fn generate_synthetic_pattern(
        pattern: TestPattern,
        frame: u64,
        width: u32,
        height: u32,
    ) -> FrameBuffer {
        let mut img = RgbaImage::new(width, height);
        let time = frame as f32 / 30.0; // Assume 30fps base for animation speed

        match pattern {
            TestPattern::SmpteColorBars => {
                // 7 standard vertical SMPTE bars
                let colors: [[u8; 4]; 7] = [
                    [204, 204, 204, 255], // 75% White
                    [204, 204, 0, 255],   // Yellow
                    [0, 204, 204, 255],   // Cyan
                    [0, 204, 0, 255],     // Green
                    [204, 0, 204, 255],   // Magenta
                    [204, 0, 0, 255],     // Red
                    [0, 0, 204, 255],     // Blue
                ];

                let bar_width = width / 7;
                for y in 0..height {
                    let is_bottom_bar = y > (height * 3 / 4);
                    for x in 0..width {
                        let bar_idx = ((x / bar_width.max(1)) as usize).min(6);
                        let color = if is_bottom_bar {
                            // Bottom section: alternates black, white, blue, dark
                            if x < width / 6 {
                                [0, 0, 180, 255] // Blue
                            } else if x < width * 2 / 6 {
                                [255, 255, 255, 255] // White
                            } else if x < width * 3 / 6 {
                                [40, 0, 60, 255] // Deep Purple
                            } else {
                                [16, 16, 16, 255] // Dark Gray
                            }
                        } else {
                            colors[bar_idx]
                        };
                        img.put_pixel(x, y, Rgba(color));
                    }
                }
            }
            TestPattern::NeonGradientMotion => {
                // Moving multi-point neon gradient (CapCut / TikTok aesthetic)
                let t = time * 1.5;
                let cx1 = (0.5 + 0.3 * (t * 0.8).cos()) * width as f32;
                let cy1 = (0.5 + 0.3 * (t * 0.6).sin()) * height as f32;

                let cx2 = (0.5 + 0.3 * (t * 0.7 + 2.0).sin()) * width as f32;
                let cy2 = (0.5 + 0.3 * (t * 0.9 + 1.0).cos()) * height as f32;

                for y in 0..height {
                    let yf = y as f32;
                    for x in 0..width {
                        let xf = x as f32;
                        let d1 =
                            ((xf - cx1).hypot(yf - cy1) / (width as f32 * 0.7)).clamp(0.0, 1.0);
                        let d2 =
                            ((xf - cx2).hypot(yf - cy2) / (width as f32 * 0.7)).clamp(0.0, 1.0);

                        // Cyan to Neon Magenta/Purple mix
                        let r = ((1.0 - d1) * 0.0 + (1.0 - d2) * 230.0) as u8;
                        let g = ((1.0 - d1) * 229.0 + (1.0 - d2) * 20.0) as u8;
                        let b = ((1.0 - d1) * 255.0 + (1.0 - d2) * 240.0) as u8;

                        img.put_pixel(
                            x,
                            y,
                            Rgba([r.clamp(18, 255), g.clamp(18, 255), b.clamp(28, 255), 255]),
                        );
                    }
                }
            }
            TestPattern::CountdownLeader => {
                // Classic film countdown leader with animated rotating sweep hand
                let cx = width as f32 / 2.0;
                let cy = height as f32 / 2.0;
                let radius = (width.min(height) as f32 * 0.38).max(10.0);
                let angle = (time * std::f32::consts::TAU) % std::f32::consts::TAU;

                for y in 0..height {
                    let yf = y as f32;
                    for x in 0..width {
                        let xf = x as f32;
                        let dx = xf - cx;
                        let dy = yf - cy;
                        let dist = dx.hypot(dy);

                        let mut color = [24, 24, 28, 255]; // Background dark

                        if (dist - radius).abs() < 4.0 || (dist - radius * 0.8).abs() < 2.0 {
                            color = [255, 255, 255, 255]; // Circles
                        } else if dist < radius {
                            // Crosshairs
                            if (dx.abs() < 2.0 && dy.abs() < radius)
                                || (dy.abs() < 2.0 && dx.abs() < radius)
                            {
                                color = [200, 200, 200, 255];
                            } else {
                                // Rotating clock sector
                                let pixel_angle =
                                    (dy.atan2(dx) + std::f32::consts::PI) % std::f32::consts::TAU;
                                if pixel_angle < angle {
                                    color = [40, 50, 70, 255];
                                }
                            }
                        }

                        img.put_pixel(x, y, Rgba(color));
                    }
                }
            }
            TestPattern::CyberGrid => {
                // Retro synthwave perspective grid
                for y in 0..height {
                    let horizon = height / 2;
                    let is_sky = y < horizon;

                    for x in 0..width {
                        let color = if is_sky {
                            // Sky gradient dark blue to deep purple
                            let factor = y as f32 / horizon as f32;
                            [
                                (10.0 + factor * 40.0) as u8,
                                (10.0 + factor * 15.0) as u8,
                                (25.0 + factor * 60.0) as u8,
                                255,
                            ]
                        } else {
                            // Floor grid lines
                            let row = y - horizon;
                            let depth = row as f32 / (height - horizon) as f32;
                            let line_y = (row % 16 == 0) as u8;
                            let persp_x =
                                ((x as i32 - width as i32 / 2) as f32 / depth.max(0.1)) as i32;
                            let line_x = (persp_x.abs() % 40 < 2) as u8;

                            if line_y == 1 || line_x == 1 {
                                [0, 242, 254, 255] // Neon Cyan grid line
                            } else {
                                [14, 14, 18, 255] // Grid floor
                            }
                        };
                        img.put_pixel(x, y, Rgba(color));
                    }
                }
            }
            TestPattern::AudioWaveformPulse => {
                // Animated oscilloscope waveform
                let mid_y = height as f32 / 2.0;
                for y in 0..height {
                    let yf = y as f32;
                    for x in 0..width {
                        let xf = x as f32;
                        let wave = (xf * 0.03 + time * 6.0).sin() * 0.4
                            + (xf * 0.08 - time * 3.0).cos() * 0.3
                            + (xf * 0.01 + time * 1.5).sin() * 0.3;
                        let wave_y = mid_y + wave * (height as f32 * 0.3);
                        let dist = (yf - wave_y).abs();

                        let color = if dist < 3.0 {
                            [0, 255, 180, 255] // Bright neon green/cyan
                        } else if dist < 8.0 {
                            [0, 100, 80, 255] // Glow
                        } else {
                            [12, 12, 16, 255] // Dark background
                        };
                        img.put_pixel(x, y, Rgba(color));
                    }
                }
            }
        }

        FrameBuffer {
            width,
            height,
            data: img.into_raw(),
        }
    }

    fn generate_text_placeholder(
        _text: &str,
        _size: f32,
        color: [f32; 4],
        width: u32,
        height: u32,
    ) -> FrameBuffer {
        // Transparent frame buffer for text overlay layer
        let mut fb = FrameBuffer::new_empty(width, height);
        let r = (color[0] * 255.0) as u8;
        let g = (color[1] * 255.0) as u8;
        let b = (color[2] * 255.0) as u8;
        let a = (color[3] * 255.0) as u8;

        // Render a clean centered subtitle banner
        let banner_h = height / 6;
        let start_y = height * 3 / 4;
        for y in start_y..(start_y + banner_h).min(height) {
            for x in (width / 6)..(width * 5 / 6) {
                let idx = ((y * width + x) * 4) as usize;
                if idx + 3 < fb.data.len() {
                    fb.data[idx] = r;
                    fb.data[idx + 1] = g;
                    fb.data[idx + 2] = b;
                    fb.data[idx + 3] = a;
                }
            }
        }

        fb
    }
}
