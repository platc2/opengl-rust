#version 450 core

in vec2 Frag_UV;
in vec4 Frag_Color;

layout (binding = 1) uniform sampler2D Texture;

layout (location = 0) out vec4 Out_Color;


void main() {
    Out_Color = Frag_Color * texture(Texture, Frag_UV.st);
}