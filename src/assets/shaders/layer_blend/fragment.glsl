#version 130

uniform sampler2D u_top_tex;
uniform sampler2D u_bottom_tex;

uniform vec2 rect_size;
uniform vec2 uv_rect_size;

float fastSqrt(float x) {
    return 1.0 / inversesqrt(x);
}

void main() {
    vec2 uv = gl_TexCoord[0].xy;
    vec4 bottom_col = texture2D(u_bottom_tex, uv);
    vec4 top_col = texture2D(u_top_tex, uv);
    // undo the opengl blending functions onto the 0,0,0,0 framebuffer
    top_col.a = fastSqrt(top_col.a);
    top_col.rgb = max(top_col.rgb / top_col.a, 0.0); // uses max so that if a division by 0 occurs, it will be 0 instead of NaN
    // re-blend the textures
    gl_FragColor = vec4(mix(bottom_col.rgb, top_col.rgb, top_col.a), fastSqrt(top_col.a) + bottom_col.a);

    // debug grid
    // ivec2 cellCoord = ivec2(floor(gl_FragCoord.xy / rect_size));
    // gl_FragColor += vec4(float(cellCoord.x) / 16, float(cellCoord.y) / 16, 0.1, 1   );
}

