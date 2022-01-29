#version 410 core

/**
 * TODO: Set up a system for declaring the types in a separate file
 *
 * An option might be to attempt to concatenate the two buffers at the point of
 * reading the GLSL files. Perhaps I could change the API to read a Vec<Path>
 * of files, in order?
 */


/**
 * TODO: Render distance fog (just compute the distance from the camera and derive from that)
 * (Possibly use a triangle function starting from (RENDER_DISTANCE_CHUNKS-1) * chunk width)
 */


#define NUM_POINT_LIGHT_SOURCES 4


/**
 * Information about the point lights in the scene
 */
struct PointLights {
    vec3 positions[NUM_POINT_LIGHT_SOURCES];
    vec3 colours[NUM_POINT_LIGHT_SOURCES];
    float intensities[NUM_POINT_LIGHT_SOURCES];
};

/**
 * Information about the global illuminant of the scene
 */
struct GlobalIlluminant {
    vec3 direction;
    vec3 colour;
    float intensity;
};


in vec4 WorldPosition;
in vec4 Normal;
in vec2 TexCoord;

uniform GlobalIlluminant globalIlluminant;
uniform PointLights pointLights;
uniform sampler2D modelTexture;

out vec4 FragColor;


/**
 * Determines the base colour by sampling the cubes texture
 */
vec4 baseColour()
{
    return texture(modelTexture, TexCoord);
}


/**
 * Calculates the additive irradiance component resulting from the scene's
 * global illuminant.
 */
vec3 irradianceFromGlobalIlluminant()
{
    float cosTheta = dot(Normal.xyz, globalIlluminant.direction);
    return globalIlluminant.intensity
         * globalIlluminant.colour
         * max(cosTheta, 0.0);
}

/**
 * Calculates the additive irradiance component resulting from one of the
 * scene's point light sources.
 */
vec3 irradianceFromPointLight(int i)
{
    vec3 position = pointLights.positions[i];
    vec3 colour = pointLights.colours[i];
    float intensity = pointLights.intensities[i];

    vec3 toLight = position - WorldPosition.xyz;
    float coefficient = max(dot(normalize(toLight), Normal.xyz), 0.0);
    float dist = clamp(length(toLight), 0.0001, 1000.0);

    return coefficient * intensity * colour / (dist*dist);
}

vec3 toneMap(vec3 colourHDR)
{
    return colourHDR / (vec3(1.0) + colourHDR);
}

vec3 gammaEncode(vec3 colour)
{
    return pow(colour, vec3(1.0 / 2.2));
}

void main()
{
    vec3 irradiance = vec3(0.0);

    // Global illumination
    float ratio = 0.8;
    irradiance += ratio * irradianceFromGlobalIlluminant() + (1.0 - ratio) * vec3(1.0);

    // Local illumination from each point light source
    for (int i = 0; i < NUM_POINT_LIGHT_SOURCES; i++) {
        irradiance += irradianceFromPointLight(i);
    }

    vec4 base = baseColour();
    vec3 radiance = base.rgb * irradiance;

    // Prepare colours for displaying
    vec3 toneMapped = toneMap(radiance);
    vec3 gammaEncoded = gammaEncode(toneMapped);
    FragColor = vec4(gammaEncoded, base.a);
}


