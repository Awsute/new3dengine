#version 460
struct Light
{
    vec3 position;
    vec3 direction;
    vec4 color;
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
in vec2 texCoord;
in vec4 fragPos;

in vec3 aPos;
uniform mat4 lookAt;
uniform mat4 mvp;

uniform Light[64] lights;
uniform Material mtl;


out vec4 FragColor;

uniform sampler2D ourTexture;


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

            vec4 lightToFrag = normalize(vec4(light.position,0.0) - fragPos);
            vec4 reflectedDir = reflect(-lightToFrag, ourNormal);


            float dp = max(dot(ourNormal, lightToFrag),0.0);
            float r = max(dot(reflectedDir, vertToCam),0.0);

            r = pow(r,mtl.shininess);

            lightColors += light.strength*light.color*((mtl.specular*r)+(dp*mtl.diffuse));
        }
    }
    
    FragColor = texture(ourTexture, texCoord)*(lightColors/lightCount)+mtl.ambient;
}
