#version 120

uniform sampler2D u_texture;

void main() {
    float distance = texture2D(u_texture, gl_TexCoord[0].xy).a;

    gl_FragColor = vec4(1.0, 1.0, 1.0, distance);
}