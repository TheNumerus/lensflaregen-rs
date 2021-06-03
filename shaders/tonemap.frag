uniform sampler2D hdr_buffer;

layout (location = 0) in vec2 uv;

out vec4 FragColor;

void main() {
    FragColor = texture(hdr_buffer, uv);
}
