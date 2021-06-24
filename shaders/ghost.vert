uniform mat4 modelMatrix;
uniform mat4 rotationMatrix;
uniform float aspect_ratio = 1.7;
uniform float ratio = 1.0;

layout (location = 0) in vec2 position;
layout (location = 1) in float vertColor;

layout (location = 0) out vec2 posInterp;
layout (location = 1) out float colorInterp;

void main() {
    posInterp = position;
    colorInterp = vertColor;
    vec4 pos_post_rotation = vec4(position, 0.0, 1.0) * rotationMatrix;
    gl_Position = modelMatrix * vec4(pos_post_rotation.xy * vec2(1.0, aspect_ratio) * vec2(1.0 / ratio, 1.0), 0.0, 1.0);
}
