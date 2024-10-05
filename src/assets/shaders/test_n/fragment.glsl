#version 120

uniform sampler2D u_texture;

void main() {
    vec4 texColor = texture2D(u_texture, gl_TexCoord[0].xy);
//    if (texColor.r == 0.0 && texColor.g == 0.0 && texColor.b == 0.0) {
//        gl_FragColor = vec4(gl_TexCoord[0].x, gl_TexCoord[0].y, 1, 1);  // Output red if the texture is black
//    } else {
//        gl_FragColor = texColor;
//    }
    gl_FragColor = texture2D(u_texture, gl_TexCoord[0].xy);
//    gl_FragColor = vec4(gl_TexCoord[0].x, gl_TexCoord[0].y, 1, 1);
}
