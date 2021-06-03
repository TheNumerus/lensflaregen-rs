#define E 2.71828
#define PI 3.14159

float gauss(float x, float center, float std_dev) {
    return pow(E, -(pow(x - center, 2.0) / std_dev));
}

vec2 euler_to_polar(vec2 euler) {
    float dist = sqrt( pow(euler.x, 2.0) + pow(euler.y, 2.0) ); // [0.0; 1.0]

    // angle component of polar coordinates
    float angle = acos(euler.x / dist);
    if (euler.y < 0.0) {
        angle = -acos(euler.x / dist);
    }

    return vec2(dist, angle);
}

vec3 encodeSRGB(vec3 linearRGB) {
    vec3 a = 12.92 * linearRGB;
    vec3 b = 1.055 * pow(linearRGB, vec3(1.0 / 2.4)) - 0.055;
    vec3 c = step(vec3(0.0031308), linearRGB);
    return mix(a, b, c);
}