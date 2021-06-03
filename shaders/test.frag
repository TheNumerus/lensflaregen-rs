uniform vec4 color;

layout (location = 0) in vec3 colorFrag;
out vec4 FragColor;

void main() {
    FragColor = color * vec4(colorFrag, 1.0);
}