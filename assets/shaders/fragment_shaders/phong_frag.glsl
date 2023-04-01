#version 460
struct Light
{
    vec3 position;
    vec3 direction;
    vec4 color;
    mat4 lookAt;
    mat4 projection;
    float strength;
};

struct Material
{
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    float shininess;  
};

smooth in vec4 ourNormal;
smooth in vec4 vertToCam;
smooth in vec2 texCoord;
in vec4 fragPos;

uniform mat4 lookAt;
uniform mat4 projection;
uniform mat4 mvp;

uniform Light[32] lights;
uniform Material mtl;


out vec4 FragColor;

uniform sampler2D depthMaps;
uniform sampler2D ourTexture;

uniform vec3 cameraDirection;
uniform vec3 cameraPosition;

void main() 
{
    vec4 lightColors = vec4(0.0);
    vec4 ourNormal = normalize(ourNormal);
    vec4 vertToCam = normalize(vertToCam);
    int lightCount = 0;
    for (int i = 0; i < lights.length(); i++) {

        if (lights[i].strength > 0.0) {
            lightCount++;
            Light light = lights[i];
            vec4 lightViewPos = light.projection * light.lookAt * fragPos;
            vec3 projCoords = (lightViewPos.xyz / lightViewPos.w) * 0.5 + 0.5;
            
            float closestDepth = texture(depthMaps, projCoords.xy).z;
            
            float currentDepth = projCoords.z;
            float bias = 0.00005 * (tan(acos(dot(ourNormal, -vec4(light.direction,1.0)))));
            float shadow = currentDepth - bias <= closestDepth ? 1.0 : 0.0;
            if (projCoords.x > 1.0 || projCoords.x < 0.0 || projCoords.y > 1.0 || projCoords.y < 0.0) {
                shadow = 0.0;
            }
            vec4 lightToFrag = normalize(vec4(light.position,0.0) - fragPos);
            vec4 reflectedDir = reflect(-lightToFrag, ourNormal);


            float dp = max(dot(ourNormal, lightToFrag), 0.0);
            float r = max(dot(reflectedDir, vertToCam), 0.0);

            r = pow(r,mtl.shininess);

            lightColors += shadow*light.strength*light.color*((mtl.specular*r)+(dp*mtl.diffuse));
        }
    }
    
    FragColor = texture(ourTexture, texCoord)*(lightColors/lightCount)+mtl.ambient;
}
