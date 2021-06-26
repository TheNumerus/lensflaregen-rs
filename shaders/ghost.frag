uniform vec4 color;
uniform float empty;

layout (location = 0) in vec2 posInterp;
layout (location = 1) in float colorInterp;

out vec3 FragColor;

void main() {
    float center = sqrt(pow(posInterp.x, 2.0) + pow(posInterp.y, 2.0));
    float edge;
    if (empty < 1.0) {
        edge = (1.0 - pow(colorInterp, 40.0) - (gauss(center, 0.0, 0.3)) * empty);
    } else {
        edge = (1.0 - pow(colorInterp, 40.0) - (gauss(pow(center, empty), 0.0, 0.3)));
    }
    FragColor = vec3(color.xyz * edge);
}