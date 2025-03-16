#version 330 core

in vec3 barycentricCoord;
in vec3 faceColor;
out vec4 fragColor;

void main() {
    // Calculate distance to the nearest edge using barycentric coordinates
    float minBary = min(min(barycentricCoord.x, barycentricCoord.y), barycentricCoord.z);
    
    float edgeThreshold = 0.02;
    vec3 edgeColor = vec3(0.0, 0.0, 0.0);
    
    // Mix face color and edge color based on distance from the edge
    if (minBary < edgeThreshold) {
        // Apply edge color
        fragColor = vec4(edgeColor, 1.0);
    } else {
        // Apply face color
        fragColor = vec4(faceColor, 1.0);
    }
}
