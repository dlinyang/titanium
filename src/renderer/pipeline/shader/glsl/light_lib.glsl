struct Light {
    bool is_position;
    bool is_range;
    vec3 color;
    vec3 position;
    vec3 direction;
    float cut_off;
    float outer_cut_off;
    float linear;
    float quadratic;
};

uniform Light[LIGHTS_MAX_NUMBER] light;
uniform int lights_count;
uniform vec3 view_position;

vec3 light_direction(Light light, vec3 frag_position) {
    if(light.is_position){
        return normalize(light.position - frag_position);
    } else {
        return normalize(-light.direction);
    }
}

vec3 light_color(Light light, vec3 direction) {
    if(light.is_range) {
        float theta = dot(-direction,normalize(light.direction));
        float epsilon = light.cut_off - light.outer_cut_off;
        float intensity = clamp( (theta - light.outer_cut_off) / epsilon, 0.0, 1.0);
        return light.color * intensity;
    } else {
        return light.color;
    }
}

float attenuation(Light light, vec3 frag_position) {
    if(light.is_position) {
        float distance = length(light.position - frag_position);
        return 1.0f / (1.0 + light.linear * distance + light.quadratic * distance * distance);
    } else {
        return 1.0;
    }
}