#version 410 core

in vec4 Position_world;
in vec4 rotatedNormal;

out vec4 FragColor;

void main()
{
    vec3 normalisedColour = vec3(0.5, 0.5, 0.5) + 0.5 * rotatedNormal.xyz;
    FragColor = vec4(normalisedColour, 1.0);
}
