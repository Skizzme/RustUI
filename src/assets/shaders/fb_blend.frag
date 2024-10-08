#version 120

uniform sampler2D u_top_tex;
uniform sampler2D u_bottom_text;

void main() {
    vec2 uv = gl_TexCoord[0].xy;
    vec4 top_col = texture2D(u_top_tex, vec2(uv.x, uv.y));
    vec4 bottom_col = texture2D(u_bottom_text, vec2(uv.x, uv.y));
    // because top_col * alpha is already done, and the framebuffer is all zeroes,
    // the only thing left to do is add the rest of the blend equation
    gl_FragColor = top_col.rgba + bottom_col.rgba * (1.0 - top_col.a);
}

