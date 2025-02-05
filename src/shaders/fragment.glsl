#version 460

layout(location = 0) in vec3 in_normal;
layout(location = 1) in vec3 in_color;
layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.0, 1.0, 1.0);

void main() {
    float brightness = dot(normalize(in_normal), normalize(LIGHT));
    vec3 dark_color = in_color * 0.75;
    vec3 regular_color = in_color;

    f_color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}