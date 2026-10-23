/// WGSL shader for 2D transform, aspect-ratio letterboxing, and CapCut-style color grading.
pub const VIDEO_PIPELINE_WGSL: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

struct TransformUniform {
    // 2x2 matrix elements + 2D translation
    transform_mat: mat2x2<f32>,
    translation: vec2<f32>,
    opacity: f32,
    aspect_ratio: f32,
};

struct ColorUniform {
    exposure: f32,
    brightness: f32,
    contrast: f32,
    saturation: f32,
    temperature: f32,
    tint: f32,
    vignette: f32,
    _padding: f32,
};

@group(0) @binding(0)
var<uniform> u_transform: TransformUniform;

@group(0) @binding(1)
var<uniform> u_color: ColorUniform;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(1) @binding(1)
var s_diffuse: sampler;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    // Apply 2D scale and rotation
    let transformed_pos = u_transform.transform_mat * model.position + u_transform.translation;
    
    out.clip_position = vec4<f32>(transformed_pos.x, transformed_pos.y, 0.0, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    // 1. Exposure adjustment (stops)
    color = vec4<f32>(color.rgb * pow(2.0, u_color.exposure), color.a);

    // 2. Brightness adjustment
    color = vec4<f32>(color.rgb + vec3<f32>(u_color.brightness), color.a);

    // 3. Contrast adjustment
    color = vec4<f32>((color.rgb - vec3<f32>(0.5)) * u_color.contrast + vec3<f32>(0.5), color.a);

    // 4. Saturation adjustment (Rec.709 luma weights)
    let luma = dot(color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let gray = vec3<f32>(luma);
    color = vec4<f32>(mix(gray, color.rgb, u_color.saturation), color.a);

    // 5. Temperature (cool/warm) & Tint (green/magenta)
    let temp_offset = vec3<f32>(u_color.temperature * 0.1, 0.0, -u_color.temperature * 0.1);
    let tint_offset = vec3<f32>(u_color.tint * 0.05, -u_color.tint * 0.05, u_color.tint * 0.05);
    color = vec4<f32>(color.rgb + temp_offset + tint_offset, color.a);

    // 6. Vignette
    let center_dist = distance(in.tex_coords, vec2<f32>(0.5, 0.5));
    let vignette_factor = 1.0 - smoothstep(0.4, 0.85, center_dist) * u_color.vignette;
    color = vec4<f32>(color.rgb * vignette_factor, color.a);

    // 7. Overall Opacity
    color = vec4<f32>(color.rgb, color.a * u_transform.opacity);

    return clamp(color, vec4<f32>(0.0), vec4<f32>(1.0));
}
"#;
