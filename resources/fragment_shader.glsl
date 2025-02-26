#version 330 core

in vec2 uv;
out vec4 FragColor;

void main() {
    // Thickness of the black outline in UV units
    float edgeThickness = 0.26;
    
    if (uv.x < edgeThickness || uv.x > 1.0 - edgeThickness || uv.y < edgeThickness || uv.y > 1.0 - edgeThickness) {
        FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        FragColor = vec4(0.0, 0.0, 1.0, 1.0);
    }
}
