#version 120

varying vec2 uvDims;
varying vec4 fragColor;
uniform sampler2D u_texture;
uniform float u_res;

void main() {
    float width = gl_TexCoord[0].z;
    float height = gl_TexCoord[0].w;

    float smoothing = (1.0 / width + 1.0 / height) / 2 * (u_res / 48) * 1.7;

    float distance = texture2D(u_texture, gl_TexCoord[0].xy).a;
    float alpha = smoothstep(0.5 - smoothing, 0.5 + smoothing, distance);

    // subpixel processing cannot be done in shader as the alphas need to be calculated
    // in a way that takes the neighbouring pixels into account in order to keep the target color.
    // for example, if the "stroke" of a glyph is in between the red and green pixel, this shader would
    // make the output a yellow-ish colour, but a proper subpixel processor would ensure that it also
    // maintains the blue-subpixel somewhere
//    float subpixel_amount = uvDims.x / (width * 3);
//
//    float r_distance = texture2D(u_texture, gl_TexCoord[0].xy + vec2(-subpixel_amount, 0)).a;
//    float r_alpha = r_distance;
//
//    float g_distance = texture2D(u_texture, gl_TexCoord[0].xy).a;
//    float g_alpha = g_distance;
//
//    float b_distance = texture2D(u_texture, gl_TexCoord[0].xy + vec2(subpixel_amount, 0)).a;
//    float b_alpha = b_distance;
//
//    float increase = 1.9;
//
//    vec3 subpixeled = vec3(fragColor.r * r_distance, fragColor.g * g_distance, fragColor.b * b_distance);
//    gl_FragColor = vec4(subpixeled * increase, fragColor.a * alpha);
    gl_FragColor = vec4(fragColor.rgb, fragColor.a * alpha);
}