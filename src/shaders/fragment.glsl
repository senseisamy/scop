#version 460

layout(location = 1) in vec3 in_pos_world;
layout(location = 2) in vec3 in_normal_world;
layout(location = 3) in vec3 in_color;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
    vec3 light_pos;
    vec4 light_color;
    vec4 ambient_light_color;
} uniforms;

void main() {
    vec3 direction_to_light = uniforms.light_pos - in_pos_world;
    float attenuation = 1.0 / dot(direction_to_light, direction_to_light);

    vec3 light_color = uniforms.light_color.xyz * uniforms.light_color.w;
    vec3 ambient_light = uniforms.ambient_light_color.xyz * uniforms.ambient_light_color.w;
    vec3 diffuse_light = light_color * max(dot(normalize(in_normal_world), normalize(direction_to_light)), 0);

    out_color = vec4((diffuse_light + ambient_light) * in_color, 1.0);
}