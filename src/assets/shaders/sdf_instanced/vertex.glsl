#version 120

attribute vec4 instanceColor;
attribute vec4 dims;
attribute vec4 uvs;
attribute int indexA;
varying vec4 charColor;

void main() {
    if (indexA == 0) {
        gl_TexCoord[0] = vec4(uvs.x, uvs.y+uvs.w, 0.0, 0.0);
        gl_Position = gl_ModelViewProjectionMatrix * vec4(dims.x, dims.y+dims.w, 0.0, 0.0);
    }
    if (indexA == 1) {
        gl_TexCoord[0] = vec4(uvs.x+uvs.z, uvs.y+uvs.w, 0.0, 0.0);
        gl_Position = gl_ModelViewProjectionMatrix * vec4(dims.x+dims.z, dims.y+dims.w, 0.0, 0.0);
    }
    if (indexA == 2) {
        gl_TexCoord[0] = vec4(uvs.x+uvs.z, uvs.y, 0.0, 0.0);
        gl_Position = gl_ModelViewProjectionMatrix * vec4(dims.x+dims.z, dims.y, 0.0, 0.0);
    }
    if (indexA == 3) {
        gl_TexCoord[0] = vec4(uvs.x, uvs.y, 0.0, 0.0);
        gl_Position = gl_ModelViewProjectionMatrix * vec4(dims.x, dims.y, 0.0, 0.0);
    }

    charColor = instanceColor;
}