uniform vec4 color = vec4(0.6, 0.6, 1.0, 1.0);
uniform float size = 10.0;
uniform float intensity = 1.0;
uniform vec2 flare_position = vec2(0.5, 0.5);
uniform float aspect_ratio = 1.7;
uniform float blades = 12.0;
uniform float ray_intensity = 1.0;
uniform float rotation;
uniform float master_intensity = 1.0;
uniform bool anamorphic = false;
uniform vec2 res = vec2(1280.0 / 64.0, 720.0 / 64.0);
uniform mat2 texture_rotation;
layout (binding = 2) uniform sampler2D noise;

layout (location = 0) in vec2 uvInterp;

out vec3 FragColor;

float rays(float distance, float norm_angle) {
    float angle = norm_angle * 2.0 * PI * blades + PI;
    float distance_limit = max(1.0 - distance, 0.0);
    float ray_centers = pow(max(cos(angle), 0.0), 8.0) * distance_limit;

    return pow(ray_centers, 2.0);
}

float radial_noise(float dist, float angle) {
    return texture(noise, vec2(dist * 0.001, angle) * texture_rotation * 5.0).r;
}

void main() {
    vec2 flare_base = (uvInterp - flare_position) * vec2(aspect_ratio, 1.0);

    vec2 polar = euler_to_polar(flare_base);
    float dist = polar.x;
    float angle = polar.y;

    // normalize
    angle += PI / 2.0;
    angle = ((angle + rotation) / (2.0 * PI));

    float rad_noise = radial_noise(dist, angle);

    float noise_ring_extrusion = mix(cos(angle * 2.0 * PI * blades + PI), 1.0, 0.95);

    float blade_count_to_ray_intensity = min(max((-blades + 18.0) / 12.0, 0.0), 1.0);

    float noise_ring_intensity = gauss(dist * noise_ring_extrusion / (size / 10.0), 0.21, 0.01);
    float noise_ring = rad_noise * noise_ring_intensity;

    float flare_value;

    #if ANAMORPHIC
        float anam_ring = (noise_ring) * 0.2;
        float anam_flare = ((gauss(dist, 0.0, size / 200.0) + anam_ring) + gauss(dist, 0.0, size / 2000.0)) * intensity;

        float ray_distort = (1.0 - pow(anam_flare, 1.0) * 0.2);
        float ray_fade = max(1.0 - abs(0.4 * flare_base.x), 0.0);

        float anam_ray_base = flare_base.y * ray_distort / ray_fade;
        float anam_ray = min(1.0, max(0.0, 1.0 - 80.0 * (abs(anam_ray_base) - 0.01))) * ray_intensity;

        flare_value = max(anam_flare + anam_ray * 1.0, anam_ray) * gauss(flare_base.x, 0.0, 0.5);
    #else
        float flare = gauss(dist, 0.0, size / 100.0);

        float rays_value = mix(noise_ring, rays(dist, angle) * rad_noise, blade_count_to_ray_intensity);

        float ray_center = 2.0 * gauss(dist, 0.0, 0.02);

        flare_value = (flare * intensity) + ((rays_value + ray_center) * ray_intensity);
    #endif

    FragColor = vec3(flare_value) * color.rgb * master_intensity;
}
