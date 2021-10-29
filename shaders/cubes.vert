#version 410 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

out vec4 rotatedNormal;

void main()
{
    // TODO: Rotate normal coordinates
    vec4 modelCoords = vec4(aPos, 1.0f);
    vec4 normalCoords = vec4(aNormal*2.0 + vec3(0.5, 0.5, 0.5), 1.0f);

    rotatedNormal = normalCoords;
    gl_Position = Projection * View * Model * modelCoords;
}
