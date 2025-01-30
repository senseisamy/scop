#version 460

layout(location = 0) in vec3 position;
layout(location = 0) out vec3 color;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
    vec3 color;
} uniforms;

void main() {
    mat4 worldview = uniforms.view * uniforms.world;
    color = uniforms.color;
    gl_Position = uniforms.proj * worldview * vec4(position, 1.0);
}