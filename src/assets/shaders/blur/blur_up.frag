// authors: shadertoy

#version 120

uniform sampler2D texture, check_texture;
uniform vec2 half_pixel, offset, resolution;
uniform int check;
uniform float noise;

vec2 hash22(vec2 p) {
	vec3 p3 = fract(vec3(p.xyx) * vec3(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx+33.33);
    return fract((p3.xx+p3.yz)*p3.zy);
}

void main() {
    vec2 uv = vec2(gl_FragCoord.xy / resolution);
    vec2 jitter = offset + (hash22(gl_FragCoord.xy) * noise);
    vec4 sum = texture2D(texture, uv + vec2(-half_pixel.x * 2.0, 0.0) * jitter);
    sum += texture2D(texture, uv + vec2(-half_pixel.x, half_pixel.y) * jitter) * 2.0;
    sum += texture2D(texture, uv + vec2(0.0, half_pixel.y * 2.0) * jitter);
    sum += texture2D(texture, uv + vec2(half_pixel.x, half_pixel.y) * jitter) * 2.0;
    sum += texture2D(texture, uv + vec2(half_pixel.x * 2.0, 0.0) * jitter);
    sum += texture2D(texture, uv + vec2(half_pixel.x, -half_pixel.y) * jitter) * 2.0;
    sum += texture2D(texture, uv + vec2(0.0, -half_pixel.y * 2.0) * jitter);
    sum += texture2D(texture, uv + vec2(-half_pixel.x, -half_pixel.y) * jitter) * 2.0;
    gl_FragColor = vec4(
        sum.rgba / 12.0 + mix(0.5 / 255.0, -0.5 / 255.0, fract(sin(dot(uv.xy, vec2(12.9, 78.2))) * 43758.5))
    );
}