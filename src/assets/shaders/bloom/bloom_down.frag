// authors: shadertoy

#version 120
#define NOISE 0.5 / 255.0

uniform sampler2D texture;
uniform vec2 offset, half_pixel, resolution;

void main() {
    vec2 uv = vec2(gl_FragCoord.xy / resolution);
    vec4 sum = texture2D(texture, gl_TexCoord[0].xy);
    sum.rgb *= sum.a;
    sum *= 4.0;
    vec4 smp1 = texture2D(texture, uv - half_pixel.xy * offset);
    smp1.rgb *= smp1.a;
    sum += smp1;
    vec4 smp2 = texture2D(texture, uv + half_pixel.xy * offset);
    smp2.rgb *= smp2.a;
    sum += smp2;
    vec4 smp3 = texture2D(texture, uv + vec2(half_pixel.x, -half_pixel.y) * offset);
    smp3.rgb *= smp3.a;
    sum += smp3;
    vec4 smp4 = texture2D(texture, uv - vec2(half_pixel.x, -half_pixel.y) * offset);
    smp4.rgb *= smp4.a;
    sum += smp4;
    vec4 result = sum / 8.0;
    gl_FragColor = vec4(result.rgb / result.a + mix(NOISE, -NOISE, fract(sin(dot(uv.xy, vec2(12.9, 78.2))) * 43758.5)), result.a);
}