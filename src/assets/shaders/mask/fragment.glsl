//#version 120

//uniform sampler2D draw_texture;
//uniform sampler2D mask_texture;
//uniform sampler2D base_texture;
//
//void main() {
//
//    vec2 uv = gl_TexCoord[0].xy;
//    vec4 sourceCol = texture2D(draw_texture, vec2(uv.x, uv.y));
//    float alpha = texture2D(mask_texture, vec2(uv.x, uv.y)).r;
//    gl_FragColor = vec4(sourceCol.rgb, sourceCol.a * alpha);
//    vec2 coord = gl_TexCoord[0].xy;
//    vec4 draw = texture2D(draw_texture, coord);
//    vec4 mask = texture2D(mask_texture, coord);
//    vec4 base = texture2D(base_texture, coord);
//
//    float alpha = dot(mask.rgb, vec3(1.0))/3.0;
//    gl_FragColor = mix(base, draw, draw.a*alpha);
//}

#version 120

uniform sampler2D u_top_tex;
uniform sampler2D u_mask_tex;
uniform sampler2D u_bottom_tex;

void main() {
    vec2 uv = gl_TexCoord[0].xy;
    vec4 bottom_col = texture2D(u_bottom_tex, vec2(uv.x, uv.y));
    vec4 top_col = texture2D(u_top_tex, vec2(uv.x, uv.y));
    vec4 mask_col = texture2D(u_mask_tex, vec2(uv.x, uv.y));
    // sqrt of the top color, since it has effectively been squared by the gl blending
    // (color.rgba) * a + 0
    top_col.a = sqrt(top_col.a);
    top_col.rgb = top_col.rgb / top_col.a;
    // because top_col * alpha is already done by gl blending, and the framebuffer is all zeroes,
    // the only thing left to do is add the rest of the blend equation
//    gl_FragColor = top_col.rgba + bottom_col.rgba * (1.0 - (top_col.a * (1.0 - mask_col.r)));
    vec4 combined = top_col.rgba * (mask_col.a) + bottom_col.rgba * (1.0 - top_col.a);
    gl_FragColor = combined;
//    gl_FragColor = mix(combined, bottom_col, 1.0 - mask_col.a);
//        gl_FragColor = vec4(top_col.a, bottom_col.a, mask_col.r, 0.5);
}


