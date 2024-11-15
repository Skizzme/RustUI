#version 120

uniform sampler2D u_top_tex;
uniform sampler2D u_bottom_tex;

void main() {
    vec2 uv = gl_TexCoord[0].xy;
    vec4 bottom_col = texture2D(u_bottom_tex, vec2(uv.x, uv.y));
    vec4 top_col = texture2D(u_top_tex, vec2(uv.x, uv.y));
    // sqrt of the top color, since it has effectively been squared by the gl blending
    // (color.rgba) * a + 0
    top_col.a = sqrt(top_col.a);
    top_col.rgb = max(top_col.rgb / top_col.a, 0.0);
    // because top_col * alpha is already done by gl blending, and the framebuffer is all zeroes,
    // the only thing left to do is add the rest of the blend equation
    gl_FragColor = vec4(mix(bottom_col.rgb, top_col.rgb, top_col.a), sqrt(top_col.a) + bottom_col.a);
//    gl_FragColor = top_col + bottom_col;
//    gl_FragColor = top_col.rgba + bottom_col.rgba * (1.0 - top_col.a);
//    gl_FragColor = vec4(top_col.rgb + bottom_col.rgb * (1.0 - sqrt(top_col.a)), bottom_col.a * (1.0 - sqrt(top_col.a)) + sqrt(top_col.a));
//    gl_FragColor = vec4(1.0, 0, 0, 0.4);
//    gl_FragColor = vec4(top_col.a, bottom_col.a, top_col.r, 0.5);
}

