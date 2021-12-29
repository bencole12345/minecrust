#version 410 core

layout (location = 0) in vec3 VertexPos;

uniform mat4 Projection;
uniform mat4 View;

out vec3 TexCoords;

void main()
{
    gl_Position = Projection * View * vec4(VertexPos, 0.0);

    // Force z/w to equal 1 so that this appears at the maximum distance,
    // guaranteeing that the skybox won't be drawn over any proper geometry
    gl_Position.z = gl_Position.w;

    TexCoords = VertexPos;
}
