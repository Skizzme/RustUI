#version 130

in vec4 dims;
in vec4 uvs;
in float ind;
in uvec4 colors;

flat out uvec4 textColors;
out vec2 uvDims;

void main() {
    vec4 pos = vec4(0, 0, 0, 0);
    if (ind == 0)
        pos = vec4(dims.x, dims.y+dims.w, uvs.x, uvs.y+uvs.w);
    else if (ind == 1)
        pos = vec4(dims.x+dims.z, dims.y+dims.w, uvs.x+uvs.z, uvs.y+uvs.w);
    else if (ind == 2)
        pos = vec4(dims.x+dims.z, dims.y, uvs.x+uvs.z, uvs.y);
    else if (ind == 3)
        pos = vec4(dims.x, dims.y, uvs.x, uvs.y);

    gl_Position = gl_ModelViewProjectionMatrix * vec4(pos.xy, 0.0, 1.0);
    gl_TexCoord[0] = vec4(pos.zw, dims.zw); // dims.zw is glyph width and height
    textColors = colors;
    uvDims = vec2(uvs.z, uvs.w);
}