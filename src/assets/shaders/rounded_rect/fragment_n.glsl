#version 130

uniform float u_random;

uniform vec4 u_color_lt;
uniform vec4 u_color_rt;
uniform vec4 u_color_lb;
uniform vec4 u_color_rb;

uniform vec2 u_size;
uniform float u_radius;

in vec2 uvs;

out vec4 fragColor;

float box(vec2 center, vec2 size, float radius) {
    return length(max(abs(center) - size, 0.0)) - radius;
}

void main() {
    vec2 halfSize = u_size * .5;

    float ign = fract(52.9829189 * fract(dot(gl_FragCoord.xy, vec2(0.06711056, 0.00583715))));

    vec4 top_color = mix(u_color_lt, u_color_rt, uvs.x);
    vec4 bottom_color = mix(u_color_lb, u_color_rb, uvs.x);

    vec4 blend_color = mix(top_color, bottom_color, uvs.y);

    float avg_brightness = ((1.0 - blend_color.r) + (1.0 - blend_color.g) + (1.0 - blend_color.b)) / 3;
//    blend_color += vec4(mix(avg_color / 255 + 0.0 / 255, -(avg_color / 255 + 0.0 / 255), fract(sin(dot(uvs.xy + vec2(u_random), vec2(12.9898, 78.233))) * 43758.5453)));

    float d = box(halfSize - (uvs.xy * u_size),
                  halfSize - u_radius - 1.0,
                  u_radius);

    float edgeWidth = 1.8;
    float alpha = (1.0 - smoothstep(0.0, edgeWidth, d)) * blend_color.a;

//    float alpha = (1.0 - smoothstep(0.0, 1.0, box(halfSize - (uvs.xy * u_size), halfSize - u_radius - 1.0, u_radius))) * blend_color.a;
    fragColor = vec4(blend_color.rgb, alpha);
//    fragColor = vec4(vec3(ign), 1.);
}
