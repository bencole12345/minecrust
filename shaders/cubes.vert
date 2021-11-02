#version 410 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

out vec4 WorldPosition;
out vec4 Normal;


void main()
{
    vec4 modelCoords = vec4(aPos, 1.0f);
    vec4 normalCoords = vec4(aNormal, 0.0f);

    // The onscreen position in NDC (normalised device coordinates)
    gl_Position = Projection * View * Model * modelCoords;

    // The current vertex's location in world space
    WorldPosition = Model * modelCoords;

    // The vertex's normal in world space
    // TODO: Confirm this rotates without translating
    Normal = normalize(Model * normalCoords);
}
