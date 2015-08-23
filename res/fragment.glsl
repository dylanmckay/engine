#version 330

smooth in vec3 interpColor;

void main()
{
    gl_FragColor = vec4(interpColor.x,interpColor.y,interpColor.z,1.0);
}
