#version 460
in vec3 aPos;

uniform mat4 lightProjection;
uniform mat4 mvp;

void main()
{
    gl_Position = lightSpaceMatrix * mvp * vec4(aPos, 1.0);
} 