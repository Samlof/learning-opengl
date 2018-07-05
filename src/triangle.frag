#version 330 core
out vec4 fragColor;

in vec3 myColor;
in vec2 TexCoord;

uniform sampler2D ourTexture;

void main()
{
    fragColor = texture(ourTexture, TexCoord) * vec4(myColor, 1.0);
}