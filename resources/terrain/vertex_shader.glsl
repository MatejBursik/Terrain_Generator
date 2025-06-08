#version 330 core

layout (location = 0) in vec3 position;

uniform mat4 transform;

// Output barycentric coordinates to highlight edges
out vec3 barycentricCoord;
out vec3 faceColor;

void main() {
    gl_Position = transform * vec4(position, 1.0);
    
    // Assign barycentric coordinates based on vertex ID
    // Cycle through (1,0,0), (0,1,0), and (0,0,1) for each triangle
    int vertexID = gl_VertexID % 3;
    if (vertexID == 0) {
        barycentricCoord = vec3(1.0, 0.0, 0.0);
    } else if (vertexID == 1) {
        barycentricCoord = vec3(0.0, 1.0, 0.0);
    } else {
        barycentricCoord = vec3(0.0, 0.0, 1.0);
    }
    
    // Generate color based on height
    faceColor = vec3(0.0, position.z * 0.5 + 0.5, position.z * 0.2 + 0.8);
}