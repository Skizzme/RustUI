#version 120

uniform vec4 u_instance;

void main() {
    gl_TexCoord[0] = vec4(u_instance.xy, 0.0, 0.0); // gl_MultiTexCoord0
//    vec4 pos = vec4(u_instance.zw, 0.0, 0.0); // gl_Vertex.xy
    vec4 pos = vec4(0, 200, 0, 0);
    gl_Position = gl_ModelViewProjectionMatrix * pos;
}