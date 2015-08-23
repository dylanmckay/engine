
#version 330
#

uniform vec4 origin;

layout(location = 0) in vec4 position;
void main()
{
    gl_Position = origin + position;
}
