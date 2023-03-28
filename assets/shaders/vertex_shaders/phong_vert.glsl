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
in vec3 aPos;
in vec3 aNormal;
in vec2 aTexCoord;
uniform mat4 mvp;
uniform mat4 projection;
uniform mat4 lookAt;

uniform vec3 cameraDirection;
uniform vec3 cameraPosition;

uniform sampler2D depthMaps;

uniform sampler2D ourTexture;

uniform Light[32] lights;


smooth out vec4 ourNormal;
smooth out vec4 vertToCam;
smooth out vec2 texCoord;
out vec4 fragPos;
void main() 
{
    fragPos = mvp*vec4(aPos, 1.0);
    gl_Position = projection*lookAt*fragPos;
    ourNormal = mvp*vec4(aNormal,0.0);
    texCoord = aTexCoord;
    vertToCam = vec4(cameraPosition,1.0)-fragPos;

}