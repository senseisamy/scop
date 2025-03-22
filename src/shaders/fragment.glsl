#version 460

layout(location = 1) in vec3 in_pos_world;
layout(location = 2) in vec3 in_normal_world;
layout(location = 3) in vec3 in_color;
layout(location = 4) in vec2 in_tex_coords;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform Data {
    mat4 world;
    mat4 view;
    mat4 proj;
    vec3 light_pos;
    vec3 light_color;
    vec3 ambient_light_color;
    bool texture;
} uniforms;

layout(set = 0, binding = 1) uniform sampler s;
layout(set = 0, binding = 2) uniform texture2D tex;

void main() {
    vec3 direction_to_light = uniforms.light_pos - in_pos_world;
    float attenuation = 1.0 / dot(direction_to_light, direction_to_light);

    vec3 light_color = uniforms.light_color.xyz;
    vec3 ambient_light = uniforms.ambient_light_color.xyz;
    vec3 diffuse_light = light_color * max(dot(normalize(in_normal_world), normalize(direction_to_light)), 0);

    vec3 color;
    if (uniforms.texture) {
        color = texture(sampler2D(tex, s), in_tex_coords).xyz;
    } else {
        color = in_color;
    }

    out_color = vec4((diffuse_light + ambient_light) * color, 1.0);
}