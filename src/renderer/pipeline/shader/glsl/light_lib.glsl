struct Light {
    vec4 color_flux;
    vec4 position;
    vec4 direction_type;
    float cut_off;
    float outer_cut_off;
    float linear;
    float quadratic;
};

#define l_color color_flux.rgb
#define l_position position.xyz
#define l_direction direction_type.xyz
#define l_type direction_type.w

#define Point 0.0
#define Spot  1.0
#define Parallel 2.0

layout (std140) uniform Lights{
    Light[LIGHTS_MAX_NUMBER] light;
};

struct Shadow {
    sampler2D tex;
    mat4 mvp;
};

uniform Shadow[LIGHTS_MAX_NUMBER] shadow;
uniform int lights_count;
uniform vec3 view_position;
uniform bool hdr_enable;
uniform float gamma;

vec3 light_direction(Light light, vec3 frag_position) {
    if(light.l_type != Parallel ){
        return normalize(light.l_position - frag_position);
    } else {
        return normalize(-light.l_direction);
    }
}

vec3 light_color(Light light, vec3 direction) {
    if(light.l_type == Spot) {
        float theta = dot(-direction,normalize(light.l_direction));
        float epsilon = light.cut_off - light.outer_cut_off;
        float intensity = clamp( (theta - light.outer_cut_off) / epsilon, 0.0, 1.0);
        return light.l_color * intensity;
    } else {
        return light.l_color;
    }
}

float visibility(vec4 position, sampler2D shadow_map, float bias) {
    vec3 frag_pos = position.xyz / position.w;
    frag_pos = frag_pos * 0.5 + 0.5;
    float closet_depth = texture(shadow_map, frag_pos.xy).r;
    float current_depth = frag_pos.z;
    return current_depth > closet_depth?0.0:1.0;
}

float attenuation(Light light, vec3 frag_position) {
    if(light.l_type != Parallel) {
        float distance = length(light.l_position - frag_position);
        return 1.0f / (1.0 + light.linear * distance + light.quadratic * distance * distance);
    } else {
        return 1.0;
    }
}

vec4 hdr(vec4 color) {
    if(hdr_enable) {
        vec3 hdr_color = color.rgb;
        vec3 mapped = hdr_color / (hdr_color + vec3(1.0));
        return vec4(pow(mapped, vec3(1.0/gamma)),color.a);
    } else {
        return color;
    }
}