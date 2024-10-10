// authors: shadertoy

#version 120

uniform sampler2D texture, check_texture;
uniform vec2 offset, half_pixel, resolution;
uniform float noise;

vec2 hash22(vec2 p) {
	vec3 p3 = fract(vec3(p.xyx) * vec3(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx+33.33);
    return fract((p3.xx+p3.yz)*p3.zy);
}

void main() {
    vec2 uv = vec2(gl_FragCoord.xy / resolution);
    vec2 jitter = offset + (hash22(gl_FragCoord.xy) * noise);
    vec4 background = texture2D(check_texture, uv);
    vec4 sum = texture2D(texture, gl_TexCoord[0].xy) * 4.0;
    sum += texture2D(texture, uv - half_pixel.xy * jitter);
    sum += texture2D(texture, uv + half_pixel.xy * jitter);
    sum += texture2D(texture, uv + vec2(half_pixel.x, -half_pixel.y) * jitter);
    sum += texture2D(texture, uv - vec2(half_pixel.x, -half_pixel.y) * jitter);
    vec4 result = sum / 8.0;
//    gl_FragColor = vec4(sum.rgb / 8.0 + mix(0.5 / 255.0, -0.5 / 255.0, fract(sin(dot(uv.xy, vec2(12.9, 78.2))) * 43758.5)), 1.0);
//    gl_FragColor = vec4(result.rgb + background.rgb * (1.0 - result.a), 1.0);
    gl_FragColor = result;
}