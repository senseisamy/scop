#version 460

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_texture;

layout(location = 0) out vec3 out_normal;
layout(location = 1) out vec3 out_color;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
    vec3 color;
} uniforms;

void main() {
    mat4 worldview = uniforms.view * uniforms.world;
    out_normal = transpose(inverse(mat3(worldview))) * in_normal;
    out_color = uniforms.color;
    gl_Position = uniforms.proj * worldview * vec4(in_position, 1.0);
}
