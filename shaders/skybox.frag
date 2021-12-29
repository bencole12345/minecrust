#version 410 core


#define PI 3.1415926535
#define EPSILON 0.000001


in vec3 TexCoords;

out vec4 FragColour;

float calculateElevation()
{
    float r = max(length(TexCoords.xz), EPSILON);
    return atan(TexCoords.y / r);
}

void main()
{
    vec3 topColour = vec3(28.0/255.0, 17.0/255.0, 188.0/255.0);
    vec3 middleColour = vec3(168.0/255.0, 226.0/255.0, 231.0/255.0);

    float angle = calculateElevation();
    float proportion = 1 - pow(cos(abs(angle / (PI/2))), 3.0);

    vec3 mixedColour = mix(middleColour, topColour, proportion);

    FragColour = vec4(mixedColour, 1.0);
}
