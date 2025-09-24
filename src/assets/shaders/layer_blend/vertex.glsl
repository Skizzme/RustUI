#version 130

uniform vec2 rect_size;
uniform vec2 uv_rect_size;

in vec4 positions;
in float ind;

void main() {
    vec4 pos = vec4(0, 0, 0, 0);
    if (ind == 0)
    pos = vec4(positions.x, positions.y+rect_size.y, positions.z, positions.w+uv_rect_size.y);
    else if (ind == 1)
    pos = vec4(positions.x+rect_size.x, positions.y+rect_size.y, positions.z+uv_rect_size.x, positions.w+uv_rect_size.y);
    else if (ind == 2)
    pos = vec4(positions.x+rect_size.x, positions.y, positions.z+uv_rect_size.x, positions.w);
    else if (ind == 3)
    pos = vec4(positions.x, positions.y, positions.z, positions.w);

    gl_Position = gl_ModelViewProjectionMatrix * vec4(pos.x, pos.y, 0.0, 1.0);
    gl_TexCoord[0] = vec4(pos.z, -pos.w, 0.0, 0.0);
}