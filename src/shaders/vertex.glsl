#version 460

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec3 in_color;
layout(location = 3) in vec2 in_texture;

layout(location = 1) out vec3 out_pos_world;
layout(location = 2) out vec3 out_normal_world;
layout(location = 3) out vec3 out_color;
layout(location = 4) out vec2 out_tex_coords;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
    vec3 light_pos;
    vec3 light_color;
    vec3 ambient_light_color;
} uniforms;

void main() {
    vec4 position_world = uniforms.world * vec4(in_position, 1.0);
    gl_Position = uniforms.proj * uniforms.view * position_world;
    out_pos_world = position_world.xyz;
    out_normal_world = normalize(mat3(uniforms.world) * in_normal);
    out_color = in_color;
    out_tex_coords = in_texture;
}
