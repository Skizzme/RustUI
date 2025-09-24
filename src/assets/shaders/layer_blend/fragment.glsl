#version 130

uniform sampler2D u_top_tex;
uniform sampler2D u_bottom_tex;

uniform sampler2D grid_mask;

uniform vec2 rect_size;
uniform vec2 uv_rect_size;

in vec2 uvDims;

void main() {
    ivec2 cellCoord = ivec2(floor(gl_FragCoord.xy / rect_size));
    float mask = texelFetch(grid_mask, cellCoord, 0).r;

    if (mask == 1) {
        vec2 uv = gl_TexCoord[0].xy;
        vec4 bottom_col = texture2D(u_bottom_tex, uv);
        vec4 top_col = texture2D(u_top_tex, uv);
        // undo the opengl blending functions onto the 0,0,0,0 framebuffer
        top_col.a = sqrt(top_col.a);
        top_col.rgb = max(top_col.rgb / top_col.a, 0.0); // uses max so that if a division by 0 occurs, it will be 0 instead of NaN

        // re-blend the textures
        gl_FragColor = vec4(mix(bottom_col.rgb, top_col.rgb, top_col.a), sqrt(top_col.a) + bottom_col.a);
    } else {
        gl_FragColor = vec4(1, 0,1,1);
    }
}

