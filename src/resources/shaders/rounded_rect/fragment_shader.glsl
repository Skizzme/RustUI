#version 120

uniform vec4 u_color;
uniform vec2 u_size;
uniform float uradius;

float box(vec2 center, vec2 size, float radius) {
    return length(max(abs(center) - size, 0.0)) - radius;
}

void main() {
    vec2 halfSize = u_size * .5;
    float alpha = (1.0 - smoothstep(0.0, 1.0, box(halfSize - (gl_TexCoord[0].xy * u_size), halfSize - uradius - 1.0, uradius))) * u_color.a;
    gl_FragColor = vec4(u_color.rgb, alpha);
}
