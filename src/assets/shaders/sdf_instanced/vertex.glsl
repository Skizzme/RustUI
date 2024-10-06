#version 120

//uniform vec4 u_instance;

void main() {
//    gl_TexCoord[0] = gl_MultiTexCoord0;
//    gl_TexCoord[1] = gl_Vertex;
    gl_TexCoord[0] = vec4(gl_Vertex.zw, 0.0, 0.0);
    vec4 pos = vec4(gl_Vertex.xy, 0.0, 1.0);
    gl_Position = gl_ModelViewProjectionMatrix * pos;
}