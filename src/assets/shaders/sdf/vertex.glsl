#version 120

attribute vec4 color;
attribute vec4 dims;
attribute vec4 uvs;
attribute float ind;

varying vec4 fragColor;

void main() {
    if (ind == 0) {
        gl_TexCoord[0] = vec4(uvs.x, uvs.y+uvs.w, dims.zw);
        gl_Position = gl_ModelViewProjectionMatrix * vec4(dims.x, dims.y+dims.w, 0.0, 1.0);
    }
    if (ind == 1) {
        gl_TexCoord[0] = vec4(uvs.x+uvs.z, uvs.y+uvs.w, dims.zw);
        gl_Position = gl_ModelViewProjectionMatrix * vec4(dims.x+dims.z, dims.y+dims.w, 0.0, 1.0);
    }
    if (ind == 2) {
        gl_TexCoord[0] = vec4(uvs.x+uvs.z, uvs.y, dims.zw);
        gl_Position = gl_ModelViewProjectionMatrix * vec4(dims.x+dims.z, dims.y, 0.0, 1.0);
    }
    if (ind == 3) {
        gl_TexCoord[0] = vec4(uvs.x, uvs.y, dims.zw);
        gl_Position = gl_ModelViewProjectionMatrix * vec4(dims.x, dims.y, 0.0, 1.0);
    }
    fragColor = color;
}