use crate::video_decoder::{FrameBuffer, MediaDecoderCache};
use capbd_core::timeline::Timeline;
use log::info;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportResolution {
    Res720p,
    Res1080p,
    Res4k,
}

impl ExportResolution {
    pub fn dimensions(&self, aspect: capbd_core::timeline::AspectRatio) -> (u32, u32) {
        let (base_w, base_h) = aspect.dimensions();
        let target_h = match self {
            ExportResolution::Res720p => 720,
            ExportResolution::Res1080p => 1080,
            ExportResolution::Res4k => 2160,
        };
        let ratio = target_h as f32 / base_h as f32;
        let target_w = ((base_w as f32 * ratio) / 2.0).round() as u32 * 2; // Even width
        (target_w, target_h)
    }

    pub fn label(&self) -> &'static str {
        match self {
            ExportResolution::Res720p => "720p (Fast / Small)",
            ExportResolution::Res1080p => "1080p Full HD (Standard)",
            ExportResolution::Res4k => "4K Ultra HD (High Quality)",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExportSettings {
    pub resolution: ExportResolution,
    pub fps: u32,
    pub output_path: String,
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            resolution: ExportResolution::Res1080p,
            fps: 30,
            output_path: "capbd_export.mp4".into(),
        }
    }
}

pub struct ExportProgress {
    pub current_frame: Arc<AtomicU64>,
    pub total_frames: Arc<AtomicU64>,
    pub is_running: Arc<AtomicBool>,
    pub is_completed: Arc<AtomicBool>,
}

impl Default for ExportProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportProgress {
    pub fn new() -> Self {
        Self {
            current_frame: Arc::new(AtomicU64::new(0)),
            total_frames: Arc::new(AtomicU64::new(1)),
            is_running: Arc::new(AtomicBool::new(false)),
            is_completed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn progress_ratio(&self) -> f32 {
        let current = self.current_frame.load(Ordering::Relaxed) as f32;
        let total = self.total_frames.load(Ordering::Relaxed).max(1) as f32;
        (current / total).clamp(0.0, 1.0)
    }
}

pub struct TimelineCompositor;

impl TimelineCompositor {
    /// Renders a single composite frame of the entire timeline at `frame` into an RGBA FrameBuffer
    pub fn render_frame(
        timeline: &Timeline,
        frame: u64,
        target_width: u32,
        target_height: u32,
        cache: &MediaDecoderCache,
    ) -> FrameBuffer {
        let mut composite = FrameBuffer::new_solid(target_width, target_height, [12, 12, 16, 255]); // Dark canvas

        let active_clips = timeline.get_active_clips_at(frame);
        if active_clips.is_empty() {
            return composite;
        }

        for (track, clip) in active_clips {
            if track.is_hidden || track.is_muted && track.track_type == capbd_core::track::TrackType::Audio {
                continue;
            }

            let clip_frame = cache.get_frame(clip, frame, target_width, target_height);
            Self::blend_layer(&mut composite, &clip_frame, clip);
        }

        composite
    }

    /// Blends `source` onto `target` with alpha blending, color grading and transform
    fn blend_layer(target: &mut FrameBuffer, source: &FrameBuffer, clip: &capbd_core::clip::Clip) {
        let opacity = clip.transform.opacity.clamp(0.0, 1.0);
        if opacity <= 0.001 {
            return;
        }

        let exposure_mul = 2.0_f32.powf(clip.color_grading.exposure);
        let brightness_add = clip.color_grading.brightness * 255.0;
        let contrast_mul = clip.color_grading.contrast;
        let saturation_mul = clip.color_grading.saturation;

        let src_w = source.width as usize;
        let src_h = source.height as usize;
        let tgt_w = target.width as usize;
        let tgt_h = target.height as usize;

        let min_w = src_w.min(tgt_w);
        let min_h = src_h.min(tgt_h);

        for y in 0..min_h {
            let row_offset_src = y * src_w * 4;
            let row_offset_tgt = y * tgt_w * 4;

            for x in 0..min_w {
                let si = row_offset_src + x * 4;
                let ti = row_offset_tgt + x * 4;

                let src_a = (source.data[si + 3] as f32 / 255.0) * opacity;
                if src_a <= 0.001 {
                    continue;
                }

                // Apply color grading
                let mut r = source.data[si] as f32;
                let mut g = source.data[si + 1] as f32;
                let mut b = source.data[si + 2] as f32;

                r = ((r * exposure_mul + brightness_add - 128.0) * contrast_mul + 128.0).clamp(0.0, 255.0);
                g = ((g * exposure_mul + brightness_add - 128.0) * contrast_mul + 128.0).clamp(0.0, 255.0);
                b = ((b * exposure_mul + brightness_add - 128.0) * contrast_mul + 128.0).clamp(0.0, 255.0);

                // Saturation
                let luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                r = (luma + (r - luma) * saturation_mul).clamp(0.0, 255.0);
                g = (luma + (g - luma) * saturation_mul).clamp(0.0, 255.0);
                b = (luma + (b - luma) * saturation_mul).clamp(0.0, 255.0);

                // Alpha blend over target
                let inv_a = 1.0 - src_a;
                target.data[ti] = (r * src_a + target.data[ti] as f32 * inv_a) as u8;
                target.data[ti + 1] = (g * src_a + target.data[ti + 1] as f32 * inv_a) as u8;
                target.data[ti + 2] = (b * src_a + target.data[ti + 2] as f32 * inv_a) as u8;
            }
        }
    }

    /// Exports a single frame snapshot as a PNG image
    pub fn export_snapshot(
        timeline: &Timeline,
        frame: u64,
        output_file: &Path,
        cache: &MediaDecoderCache,
    ) -> Result<(), String> {
        let (w, h) = timeline.aspect_ratio.dimensions();
        let fb = Self::render_frame(timeline, frame, w, h, cache);

        if let Some(img) = image::RgbaImage::from_raw(fb.width, fb.height, fb.data) {
            img.save(output_file).map_err(|e| e.to_string())?;
            info!("Exported snapshot to {}", output_file.display());
            Ok(())
        } else {
            Err("Failed to create image from frame buffer".into())
        }
    }
}
