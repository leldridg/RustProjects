#version 330 core

layout (location = 0) in vec2 Position;

uniform vec2 u_resolution;
uniform mat3 u_model_matrix;

// executed in parallel for each vertex
void main() {
    // vec2 uv = Position;
    vec3 uv = u_model_matrix * vec3(Position, 1.0);

    // make ((-1.0, -1.0), (1.0, -1.0)), (1.0, 1.0, (-1.0, 1.0)) a square always in the center of the viewport
    if (u_resolution.x > u_resolution.y) {
        uv.x *= u_resolution.y / u_resolution.x;
    } else {
        uv.y *= u_resolution.x / u_resolution.y;
    }

    gl_Position = vec4(uv, 1.0);
}