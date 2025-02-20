#version 330 core

// layout (location = 0) in vec2 Position;
layout (location = 0) in uint Entity_id;
layout (location = 1) in vec3 Position;

uniform vec2 u_resolution;
// uniform mat3 u_model_matrix;
uniform mat4 u_model_matrix[2];
uniform mat4 u_view_matrix;
uniform mat4 u_projection_matrix;

out VS_OUTPUT {
    vec3 Color;
} OUT;

// executed in parallel for each vertex
void main() {
    // vec2 uv = Position;
    // vec3 uv = u_model_matrix * vec3(Position, 1.0);
    vec4 uv = u_model_matrix[Entity_id] * vec4(Position, 1.0); // 0.0 is z, 1.0 is w
    OUT.Color = vec3(uv.x, uv.y, uv.z);
    //OUT.Color = vec3(1, 0, 0);
    uv = u_projection_matrix * u_view_matrix * uv;

    // make ((-1.0, -1.0), (1.0, -1.0)), (1.0, 1.0, (-1.0, 1.0)) a square always in the center of the viewport
    if (u_resolution.x > u_resolution.y) {
        uv.x *= u_resolution.y / u_resolution.x;
    } else {
        uv.y *= u_resolution.x / u_resolution.y;
    }

    // gl_Position = vec4(uv, 1.0);
    gl_Position = uv;
}