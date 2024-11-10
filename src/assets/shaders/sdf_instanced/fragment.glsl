#version 120

varying vec4 charColor;         // Receive color from vertex shader
uniform sampler2D u_texture;
uniform float u_smoothing;

void main() {
//    float distance = texture2D(u_texture, gl_TexCoord[0].xy).a;
//    float alpha = smoothstep(0.5 - u_smoothing, 0.5 + u_smoothing, distance);
//    gl_FragColor = vec4(charColor.rgb, charColor.a * alpha);
    gl_FragColor = vec4(1,1,1,1);
}