#version 330

const float RED_LUM_CONSTANT = 0.2126;
const float GREEN_LUM_CONSTANT = 0.7152;
const float BLUE_LUM_CONSTANT = 0.0722;

uniform sampler2D texture0;
uniform vec2 lightPosition;
uniform float time;

in vec2 fragTexCoord;
in vec2 fragPosition;

out vec4 finalColor;

// Adjustable attenuation parameters
const float LINEAR_TERM    = 0.0003; // linear term
const float QUADRATIC_TERM = 0.00001; // quadratic term
const float MIN_BRIGHTNESS = 0.05;    // avoid total darkness

float attenuate(float distance, float linearTerm, float quadraticTerm)
{
    float attenuation = 1.0 / (1.0 + 
                               linearTerm * distance + 
                               quadraticTerm * distance * distance);

    return max(attenuation, MIN_BRIGHTNESS);
}

/*
float caustics(vec2 uv) {
    vec2 p = uv * 10.0;
    float c = sin(p.x + sin(p.y + time)) * 
              sin(p.y + sin(p.x + time * 0.7));
    return c * 0.5 + 0.5;
}
*/

void main()
{
    vec2 distorted = fragTexCoord;
    distorted.y += sin(time + distorted.x * 30)/1500;
    distorted.x += sin(time + distorted.y * 30)/1500;
    vec4 color = texture(texture0, distorted);

    float dist = distance(lightPosition, fragPosition);

    float layer1 = (sin(fragPosition.x/85 + time) - sin((fragPosition.y + fragPosition.x)/60 + time)) / 4 + 0.5;
    float layer2 = (sin((fragPosition.y + fragPosition.x)/90 + 2 * time) - sin(fragPosition.x/95 + 2 * time)) / 4 + 0.5;

    float layer3 = (sin((fragPosition.y - fragPosition.x)/35 - 3 * time) - sin(fragPosition.x/30 - 3 * time)) / 4 + 0.5;
    float wiggle = 0.4 * layer1 + 0.4 * layer2 + 0.2 * layer3;

    vec4 tint1 = vec4(0.0, 0.7, .9,1.0);
    vec4 tint2 = vec4(0.0, 0.3, .7,1.0);

    vec4 overlay = mix(tint1, tint2, wiggle);
    vec4 overlayed = mix(overlay, color, attenuate(dist, LINEAR_TERM,QUADRATIC_TERM) * .5 + 0.2);

    finalColor = overlayed;
}