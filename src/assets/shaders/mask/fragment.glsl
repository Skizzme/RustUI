#version 120

uniform sampler2D draw_texture;
uniform sampler2D mask_texture;
uniform sampler2D base_texture;

void main() {
    vec2 coord = gl_TexCoord[0].xy;
    vec4 draw = texture2D(draw_texture, coord);
    vec4 mask = texture2D(mask_texture, coord);
    vec4 base = texture2D(base_texture, coord);

    float alpha = dot(mask.rgb, vec3(1.0))/3.0;
//    gl_FragColor = mix(vec4(base.rgb, 1.0), vec4(draw.rgb, 1.0), draw.a*alpha); // Applies base, allow for smoother blending
//    vec4 masked_color = vec4(vec3(1.0), alpha*draw.a);
//    gl_FragColor = mix(base, masked_color, alpha); // Applies base, allow for smoother blending
//    gl_FragColor = vec4(vec3(1.0), 0.4*alpha);
//    if (draw.r == 0.4 && draw.g == 0.4) {
//        gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
//
//    } else {
//        gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
//    }
//    gl_FragColor = vec4(1.0, 1.0, 1.0, draw.a);
//    gl_FragColor = vec4(mix(vec3(0.5647059, 0.0627451, 0.0627451), vec3(1.0, 1.0, 1.0), 0.4), 1.0); // THIS IS NORMAL BLENDING
    //vec4(vec3(1.0), alpha*0.4)
//    gl_FragColor = masked_color;
//    gl_FragColor = vec4(mix(base.rgb, draw.rgb, draw.a*alpha), 1.0);
    gl_FragColor = mix(base, draw, draw.a*alpha);
//    gl_FragColor = mix(vec4(base.rgb, 1.0), vec4(draw.rgb, 1.0), clamp(draw.a*alpha+(1.0-base.a), 0.0, 1.0)); // sorta works?
//    gl_FragColor = vec4(draw.rgb, draw.a*alpha);
//    gl_FragColor = vec4(draw.rgb, alpha);
}

