#version 120
#define NOISE 0.5 / 255.0

uniform vec2 size;
uniform vec4 color;
uniform float alias;

void main() {
    vec2 uv = gl_TexCoord[0].xy;
    vec2 temp = gl_PointCoord - vec2(0.5);
    float f = dot(temp, temp);
//    if (f>0.25) discard;
    gl_FragColor = vec4(temp.s, temp.t, 1.0, 1.0);
}