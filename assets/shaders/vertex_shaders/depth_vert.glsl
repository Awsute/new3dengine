#version 460
in vec3 aPos;

uniform mat4 lightProjection;
uniform mat4 lightLookAt;
uniform mat4 mvp;

void main()
{
    gl_Position = lightProjection * lightLookAt * mvp * vec4(aPos, 1.0);
} 