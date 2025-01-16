#version 130

uniform sampler2D u_texture;
uniform float u_res;
uniform vec2 u_atlas_dims;

flat in uvec4 textColors;
in vec2 uvDims;

vec4 unpackColor(uint packedColor) {
    float a = ((packedColor >> 24) & 0xFFu) / 255.0;
    float r = ((packedColor >> 16) & 0xFFu) / 255.0;
    float g = ((packedColor >> 8) & 0xFFu) / 255.0;
    float b = (packedColor & 0xFFu) / 255.0;
    return vec4(r, g, b, a);
}

void main() {
    float width = gl_TexCoord[0].z;
    float height = gl_TexCoord[0].w;

    // Calculate screen-space derivatives of the texture coordinates
    vec2 texCoord = gl_TexCoord[0].xy;
    vec2 screenDerivatives = vec2(length(dFdx(texCoord)), length(dFdy(texCoord)));
    vec2 normalizedDerivates = screenDerivatives * u_atlas_dims * 0.045;

    // Determine the smoothing factor (approximate the ratio of SDF pixels to screen pixels)
    // + 0.2 * (u_res / 48)
    float smoothing = max(normalizedDerivates.x, normalizedDerivates.y);

    // Adjust the SDF distance value for the zero-level and bleed
    float distance = texture2D(u_texture, texCoord).a;
    float adjustedDistance = (distance - 0.5) / smoothing + 0.5;

    // Compute alpha with smoothstep for anti-aliased edges
    float alpha = smoothstep(0.5 - smoothing, 0.5 + smoothing, distance);
//    float alpha = adjustedDistance;

    vec4 textColor = unpackColor(textColors.x);
    vec4 outlineColor = unpackColor(textColors.y);
    vec4 fragColor = textColor;

//    float outlineDistance = 0.0;
//    if (outlineColor.a > 0) {
//        if (distance < 0.5) {
//            outlineDistance = distance;
//        } else {
//            outlineDistance = 0.5 - (distance - 0.5);
//        }
//        outlineDistance = sqrt(outlineDistance) * 1.0 + outlineColor.a / 10;
//
//        float outlineAlpha = smoothstep(0.70710 - smoothing, 0.70710 + smoothing, outlineDistance);
//
//        if (outlineAlpha > 0.0) {
//            alpha = max(outlineAlpha, alpha);
//            fragColor = vec4(mix(textColor.rgb, outlineColor.rgb, outlineAlpha), textColor.a);
//        }
//    }

    gl_FragColor = vec4(fragColor.rgb, fragColor.a * alpha);
}