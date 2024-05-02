#version 120

uniform vec4 u_color;
uniform sampler2D u_texture;
uniform float u_smoothing;

void main() {
    float distance = texture2D(u_texture, gl_TexCoord[0].xy).a;
    float alpha = smoothstep(0.5 - u_smoothing, 0.5 + u_smoothing, distance);
//    gl_FragColor = vec4(u_color.rgb, 1-(u_color.a * alpha));
    gl_FragColor = vec4(u_color.rgb, u_color.a * alpha);
}
