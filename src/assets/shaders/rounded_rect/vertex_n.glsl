#version 130

in vec4 vert;

out vec2 uvs;

void main() {
    gl_Position = gl_ModelViewProjectionMatrix * vec4(vert.xy, 0.0, 1.0);
//    gl_TexCoord[0] = vec4(vert.zw, 0.0, 0.0);
    uvs = vert.zw;
}