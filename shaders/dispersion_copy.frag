layout(binding=0) uniform sampler2D ghost;
layout(binding=2) uniform sampler2D noise;
layout(binding=3) uniform sampler2D spectral;
uniform float dispersion = 0.4;
uniform float distortion = 1.0;
uniform int samples = 8;
uniform float master_intensity = 1.0;
uniform float intensity = 1.0;
uniform vec2 res = vec2(1280.0 / 64.0, 720.0 / 64.0);
uniform float use_jitter = 1.0;
uniform float disperse_from_ghost_center = 0.0;
uniform vec2 ghost_pos;

layout(location = 0) in vec2 uvInterp;

out vec3 FragColor;

vec2 uv_scaled(vec2 uv, float scale) {
    vec2 centered = uv - 0.5;
    if (disperse_from_ghost_center > 0.5) {
        centered = uv - ghost_pos / 2.0 - 0.5;
    }
    vec2 scaled = centered * scale;
    if (disperse_from_ghost_center > 0.5) {
        return scaled + ghost_pos / 2.0 + 0.5;
    }
    return scaled + 0.5;
}

vec2 distortion_vector() {
    vec2 moved = uvInterp - 0.5;
    return ( (moved.x * moved.x) + (moved.y * moved.y) ) * moved * -distortion;
}

void main() {
    vec3 color = vec3(0.0);
    float pixel_offset = texture(noise, uvInterp * res).r * use_jitter;
    vec2 pixel_distortion = uvInterp + distortion_vector();

    float x = pixel_offset / float(samples);
    float delta = 1.0 / float(samples);

    for (int i = 0; i < samples; ++i) {
        float sample_dispersion = ((x * 2.0) - 1.0) * dispersion + 1.0;
        vec4 ghost_color = texture(ghost, uv_scaled(pixel_distortion, sample_dispersion));

        vec4 spectral_tex = texture(spectral, vec2(x, 0.5));

        color += ghost_color.rgb * spectral_tex.rgb;

        x += delta;
    }

    color /= float(samples);

    FragColor = color * intensity * master_intensity;
}