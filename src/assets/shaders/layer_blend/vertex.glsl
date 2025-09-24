#version 130

uniform vec2 rect_size;
uniform vec2 uv_rect_size;
uniform ivec2 grid_dims;

in int index;
in float ind;

void main() {
    vec4 pos = vec4(0, 0, 0, 0);
//    vec2 grid_
    vec2 grid_xy = vec2(index % 16, floor(index / 16));
    vec4 grid_pos = vec4(grid_xy * rect_size, grid_xy * uv_rect_size);
    
    if (ind == 0)
        pos = vec4(grid_pos.x, grid_pos.y+rect_size.y, grid_pos.z, grid_pos.w+uv_rect_size.y);
    else if (ind == 1)
        pos = vec4(grid_pos.x+rect_size.x, grid_pos.y+rect_size.y, grid_pos.z+uv_rect_size.x, grid_pos.w+uv_rect_size.y);
    else if (ind == 2)
        pos = vec4(grid_pos.x+rect_size.x, grid_pos.y, grid_pos.z+uv_rect_size.x, grid_pos.w);
    else if (ind == 3)
        pos = vec4(grid_pos.x, grid_pos.y, grid_pos.z, grid_pos.w);

    gl_Position = gl_ModelViewProjectionMatrix * vec4(pos.x, pos.y, 0.0, 1.0);
    gl_TexCoord[0] = vec4(pos.z, -pos.w, 0.0, 0.0);
}