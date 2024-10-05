#version 120

//uniform vec4 u_instance;

void main() {
    gl_TexCoord[0] = gl_MultiTexCoord0;
    vec4 pos = gl_Vertex;
    gl_Position = gl_ModelViewProjectionMatrix * pos;
}