#version 120

uniform sampler2D u_top_tex;
uniform sampler2D u_mask_tex;
uniform sampler2D u_bottom_tex;

void main() {
    vec2 uv = gl_TexCoord[0].xy;
    vec4 bottom_col = texture2D(u_bottom_tex, uv);
    vec4 top_col = texture2D(u_top_tex, uv);
    vec4 mask_col = texture2D(u_mask_tex, uv);

    // undo the opengl blending functions onto the 0,0,0,0 framebuffer
    top_col.a = (top_col.a);
    top_col.rgb = max(top_col.rgb / top_col.a, 0.0);

    gl_FragColor = vec4(bottom_col.rgb + (top_col.rgb * mask_col.r), sqrt(top_col.a * mask_col.r) + bottom_col.a);
}


