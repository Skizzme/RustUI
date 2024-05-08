#version 120

uniform vec4 u_color;
uniform sampler2D u_texture;
//uniform float u_smoothing;
//uniform float atlas_width;
//uniform float i_scale;
uniform vec3 u_values;

void main() {
    float u_smoothing = u_values[0];
    float atlas_width = u_values[1];
    float i_scale = u_values[2];
//    float distance = texture2D(u_texture, gl_TexCoord[0].xy).a;
//    float alpha = smoothstep(0.5 - u_smoothing, 0.5 + u_smoothing, distance);
//    gl_FragColor = vec4(u_color.rgb, u_color.a * alpha);
    float rdistance = texture2D(u_texture, gl_TexCoord[0].xy).a;
    float r = smoothstep(0.5 - u_smoothing, 0.5 + u_smoothing, rdistance);
    float gdistance = texture2D(u_texture, gl_TexCoord[0].xy + vec2(i_scale / atlas_width / 3, 0.0)).a;
    float g = smoothstep(0.5 - u_smoothing, 0.5 + u_smoothing, gdistance);
    float bdistance = texture2D(u_texture, gl_TexCoord[0].xy + vec2(i_scale * 2 / atlas_width / 3, 0.0)).a;
    float b = smoothstep(0.5 - u_smoothing, 0.5 + u_smoothing, bdistance);
    gl_FragColor = vec4(u_color.rgb, u_color.a * (r+g+b)/3);
//    gl_FragColor = vec4(u_color.r * r, u_color.g * g, u_color.b * b, u_color.a * alpha);
}
