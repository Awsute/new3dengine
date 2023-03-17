#version 460
struct Light
{
    vec3 position;
    vec3 direction;
    vec4 color;
    float strength[1];
};

in vec4 ourNormal;
in vec2 texCoord;


layout (location = 5) uniform mat4 lookAt;
layout (binding = 0) uniform Lights { Light lights[4]; };



out vec4 FragColor;

uniform sampler2D ourTexture;
void main() {
    vec4 aCol = vec4(1.0);
    //for (int i = 0; i < lights.length(); i++){
    //    vec4 color = lights[i].color*lights[i].strength[0];
    //    aCol += color;
    //}
    FragColor = texture(ourTexture, texCoord)*aCol;
}
