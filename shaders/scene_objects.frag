#version 410 core

/**
 * TODO: Set up a system for declaring the types in a separate file
 *
 * An option might be to attempt to concatenate the two buffers at the point of
 * reading the GLSL files. Perhaps I could change the API to read a Vec<Path>
 * of files, in order?
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

/**
 * Parameters about the thresholds for distance fog
 */
struct FogParameters {
    float beginDistance;
    float totalDistance;
};


in vec4 WorldPosition;
in vec4 Normal;
in vec2 TexCoord;

uniform vec3 cameraPos;
uniform GlobalIlluminant globalIlluminant;
uniform PointLights pointLights;
uniform sampler2D modelTexture;
uniform FogParameters fogParameters;

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

float computeOpacityFromFog()
{
    float distanceXZ = length(WorldPosition.xz - cameraPos.xz);
    float w = (distanceXZ - fogParameters.beginDistance)
    / (fogParameters.totalDistance - fogParameters.beginDistance);
    return clamp(1.0 - w, 0.0, 1.0);
}

vec3 sampleSkybox()
{
    vec3 toFragment = WorldPosition.xyz - cameraPos;
    float r = length(toFragment.xz);
    float elevation = atan(toFragment.y / r + 0.00001);
    float proportion = 1 - pow(cos(abs(elevation)), 3.0);

    vec3 topColour = vec3(28.0/255.0, 17.0/255.0, 188.0/255.0);
    vec3 middleColour = vec3(168.0/255.0, 226.0/255.0, 231.0/255.0);

    return mix(middleColour, topColour, proportion);
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

    // Mix with distance fog
    float alpha = computeOpacityFromFog();
    vec3 bgColour = sampleSkybox();
    vec3 finalRadiance = mix(bgColour, gammaEncoded, alpha);

    FragColor = vec4(finalRadiance, alpha);
}


