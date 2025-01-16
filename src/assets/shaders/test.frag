#version 120

uniform sampler2D u_texture;

void main() {
    float distance = texture2D(u_texture, gl_TexCoord[0].xy).a;

    float smoothing = 0.1;
    float alpha = smoothstep(0.5 - smoothing, 0.5 + smoothing, distance);

    gl_FragColor = vec4(1.0, 1.0, 1.0, distance);
}