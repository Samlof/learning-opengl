#version 330 core
out vec4 fragColor;

in vec3 myColor;
in vec3 myPos;

void main()
{
    fragColor = vec4(myColor, 1.0);
}