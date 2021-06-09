layout (binding = 0) uniform sampler2D hdr_buffer;

layout (location = 0) in vec2 uv;

out vec4 FragColor;

void main() {
    FragColor = vec4(encodeSRGB(texture(hdr_buffer, uv).rgb), 1.0);
}
