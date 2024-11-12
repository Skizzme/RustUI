#version 120

varying vec4 fragColor;
uniform sampler2D u_texture;
uniform float u_res;

void main() {
    float distance = texture2D(u_texture, gl_TexCoord[0].xy).a;
    float width = gl_TexCoord[0].z;
    float height = gl_TexCoord[0].w;
    float smoothing = (1.0 / width + 1.0 / height) / 2 * 2.2 * (u_res / 48);
    float alpha = smoothstep(0.5 - smoothing, 0.5 + smoothing, distance);
    gl_FragColor = vec4(fragColor.rgb, fragColor.a * alpha);
}