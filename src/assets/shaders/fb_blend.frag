#version 120

uniform sampler2D u_top_tex;
uniform sampler2D u_bottom_tex;

void main() {
    vec2 uv = gl_TexCoord[0].xy;
    vec4 bottom_col = texture2D(u_bottom_tex, uv);
    vec4 top_col = texture2D(u_top_tex, uv);
    // undo the opengl blending functions onto the 0,0,0,0 framebuffer
    top_col.a = sqrt(top_col.a);
    top_col.rgb = max(top_col.rgb / top_col.a, 0.0);

    // re-blend the textures
    gl_FragColor = vec4(mix(bottom_col.rgb, top_col.rgb, top_col.a), sqrt(top_col.a) + bottom_col.a);
}

