#version 460
struct Light
{
    vec3 position;
    vec3 direction;
    vec4 color;
    float strength[1];
};

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;
layout (location = 3) uniform mat4 mvp;
layout (location = 4) uniform mat4 projection;
layout (location = 5) uniform mat4 lookAt;

layout (location = 6) uniform vec3 cameraDirection;
layout (location = 7) uniform vec3 cameraPosition;

out vec4 ourNormal;
out vec2 texCoord;
void main() {
    gl_Position = projection*lookAt*mvp*vec4(aPos.x,aPos.y,aPos.z, 1.0);
    ourNormal = vec4(aNormal.x, -aNormal.y, aNormal.z, 0.0);
    texCoord = aTexCoord;

}