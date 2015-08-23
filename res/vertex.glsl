
#version 330

layout(location = 0) in vec3 normal;
layout(location = 1) in vec4 position;

uniform mat4 worldTransform;
uniform vec3 lightPosition;

smooth out vec3 interpColor;

void main()
{
    gl_Position = worldTransform*position;

/*    vec3 normalisedNormal = normalize(normal);
    vec3 lightDir = vec3(gl_Position.x,gl_Position.y,gl_Position.z) - lightPosition;

    float cosTheta = clamp( dot(normalisedNormal, lightDir), 0, 1);
    interpColor = vec3(0.81,0.22,0.0)*cosTheta;*/
}
