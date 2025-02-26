#version 330 core

layout(location = 0) in vec3 position;
out vec2 uv;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(position, 1.0);
    // Map from [-1,1] to [0,1] to obtain UV coordinates
    uv = (position.xy + vec2(1.0)) * 0.5;
}
