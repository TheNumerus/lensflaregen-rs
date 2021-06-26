layout(binding=0) uniform sampler2D ghost;
layout(binding=2) uniform sampler2D noise;
uniform float dispersion = 0.4;
uniform float distortion = 1.0;
uniform int samples = 8;
uniform float master_intensity = 1.0;
uniform float intensity = 1.0;
uniform vec2 res = vec2(1280.0 / 64.0, 720.0 / 64.0);
uniform float use_jitter = 1.0;
uniform bool disperse_from_ghost_center = false;
uniform vec2 ghost_pos;

layout(location = 0) in vec2 uvInterp;

out vec3 FragColor;

vec2 uv_scaled(vec2 uv, float scale) {
    if (disperse_from_ghost_center) {
        vec2 centered = uv - ghost_pos * 0.5 - 0.5;
        vec2 scaled = centered * scale;
        return scaled + ghost_pos * 0.5 + 0.5;
    } else {
        vec2 centered = uv - 0.5;
        vec2 scaled = centered * scale;
        return scaled + 0.5;
    }
}

vec2 distortion_vector() {
    vec2 moved = uvInterp - 0.5;
    return ( (moved.x * moved.x) + (moved.y * moved.y) ) * moved * -distortion;
}

vec3 spectrum_dist(float x) {
    float r = gauss(x, 0.65, 0.03);
    float g = gauss(x, 0.5, 0.03);
    float b = gauss(x, 0.35, 0.03);

    return vec3(r, g, b);
}

void main() {
    vec3 color = vec3(0.0);
    float pixel_offset = texture(noise, uvInterp * res).r * use_jitter;
    vec2 pixel_distortion = uvInterp + distortion_vector();

    float samples_f = float(samples);
    float x = pixel_offset / samples_f;
    float delta = 1.0 / samples_f;

    for (int i = 0; i < samples; ++i) {
        float sample_dispersion = ((x * 2.0) - 1.0) * dispersion + 1.0;
        vec4 ghost_color = texture(ghost, uv_scaled(pixel_distortion, sample_dispersion));

        color += ghost_color.rgb * spectrum_dist(x);

        x += delta;
    }

    color /= samples_f;

    FragColor = color * intensity * master_intensity;
}