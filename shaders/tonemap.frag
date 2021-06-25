layout (binding = 0) uniform sampler2D hdr_buffer;
uniform int tonemap = 1;

layout (location = 0) in vec2 uv;

out vec4 FragColor;

vec3 reinhard(vec3 src) {
    return (src) / (1.0 + src);
}

void main() {
    vec4 src = texture(hdr_buffer, uv);
    if (tonemap == 1) {
        FragColor = vec4(encodeSRGB(reinhard(src.rgb)), 1.0);
    } else {
        FragColor = vec4(encodeSRGB(src.rgb), 1.0);
    }
}
