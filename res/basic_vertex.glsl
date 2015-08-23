
#version 330
#

uniform mat4 worldTransform;

layout(location = 0) in vec4 position;
layout(location = 1) in vec4 normal;
void main()
{
    gl_Position = worldTransform * position;
}
