    // authors: shadertoy

#version 120
#define NOISE 0.5 / 255.0

uniform sampler2D texture, read_texture;
uniform vec2 offset, half_pixel, resolution;
uniform int check;

void main() {
    vec2 uv = vec2(gl_FragCoord.xy / resolution);
    vec4 sum = texture2D(texture, uv + vec2(-half_pixel.x * 2.0, 0.0) * offset);
    sum.rgb *= sum.a;
    vec4 smpl1 =  texture2D(texture, uv + vec2(-half_pixel.x, half_pixel.y) * offset);
    smpl1.rgb *= smpl1.a;
    sum += smpl1 * 2.0;
    vec4 smp2 = texture2D(texture, uv + vec2(0.0, half_pixel.y * 2.0) * offset);
    smp2.rgb *= smp2.a;
    sum += smp2;
    vec4 smp3 = texture2D(texture, uv + vec2(half_pixel.x, half_pixel.y) * offset);
    smp3.rgb *= smp3.a;
    sum += smp3 * 2.0;
    vec4 smp4 = texture2D(texture, uv + vec2(half_pixel.x * 2.0, 0.0) * offset);
    smp4.rgb *= smp4.a;
    sum += smp4;
    vec4 smp5 = texture2D(texture, uv + vec2(half_pixel.x, -half_pixel.y) * offset);
    smp5.rgb *= smp5.a;
    sum += smp5 * 2.0;
    vec4 smp6 = texture2D(texture, uv + vec2(0.0, -half_pixel.y * 2.0) * offset);
    smp6.rgb *= smp6.a;
    sum += smp6;
    vec4 smp7 = texture2D(texture, uv + vec2(-half_pixel.x, -half_pixel.y) * offset);
    smp7.rgb *= smp7.a;
    sum += smp7 * 2.0;
    vec4 result = sum / 12.0;
    gl_FragColor = vec4(result.rgb / result.a, result.a);
//    gl_FragColor = vec4(result.rgb / result.a + mix(NOISE, -NOISE, fract(sin(dot(uv.xy, vec2(12.9, 78.2))) * 43758.5)), mix(result.a, result.a * (1.0 - texture2D(read_texture, gl_TexCoord[0].xy).a), check));
}