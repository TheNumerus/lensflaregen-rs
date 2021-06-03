layout (location = 0) in vec2 aPos;
layout (location = 1) in vec3 color;

layout (location = 0) out vec3 colorFrag;

void main() {
    colorFrag = color;
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
}