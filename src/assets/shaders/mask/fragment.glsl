#version 120

uniform sampler2D draw_texture;
uniform sampler2D mask_texture;
uniform sampler2D base_texture;

void main() {

    vec2 uv = gl_TexCoord[0].xy;
    vec4 sourceCol = texture2D(draw_texture, vec2(uv.x, uv.y));
    float alpha = texture2D(mask_texture, vec2(uv.x, uv.y)).r;
    gl_FragColor = vec4(sourceCol.rgb, sourceCol.a * alpha);
//    vec2 coord = gl_TexCoord[0].xy;
//    vec4 draw = texture2D(draw_texture, coord);
//    vec4 mask = texture2D(mask_texture, coord);
//    vec4 base = texture2D(base_texture, coord);
//
//    float alpha = dot(mask.rgb, vec3(1.0))/3.0;
//    gl_FragColor = mix(base, draw, draw.a*alpha);
}

