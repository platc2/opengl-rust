#version 430 core
#extension GL_ARB_enhanced_layouts : enable


layout (location = 0) in vec2 position;

layout (location = 0) out SHADER_VARYING {
    vec2 uv;
};

void main() {
    gl_Position = vec4(position, 0, 1);
    uv = (position + vec2(1, 1)) / 2;
}