#version 120

varying vec4 fragColor;
uniform sampler2D u_texture;
uniform float u_smoothing;

void main() {
    float distance = texture2D(u_texture, gl_TexCoord[0].xy).a;
    float alpha = smoothstep(0.5 - u_smoothing, 0.5 + u_smoothing, distance);
    gl_FragColor = vec4(fragColor.rgb, fragColor.a * alpha);
    //    gl_FragColor = vec4(gl_TexCoord[0].x, gl_TexCoord[0].y, 1, 1);
    //    gl_FragColor = vec4(alpha, gl_TexCoord[0].x, gl_TexCoord[0].y, 1.0);
}