#version 410 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

out vec4 WorldPosition;
out vec4 Normal;
out vec2 TexCoord;


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
    // (It's supposed to abuse whether there's a 0 or a 1 in the w dimension in the
    // homogenous coordinates representation; it's an nalgebra thing)
    Normal = normalize(Model * normalCoords);

    TexCoord = aTexCoord;
}
