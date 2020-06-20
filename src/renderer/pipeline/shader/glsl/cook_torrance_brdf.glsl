#define PI 3.14159265358979

struct CookTorranceBRDF {
    vec3 albedo;
    float roughness;
    float metallic;
    float ao;
};

uniform CookTorranceBRDF material;

in vec3 v_normal;
in vec3 frag_pos;

out vec4 color_out;

float GGX_distribution(vec3 normal, vec3 halfway_direction, float roughness) {
    float a = roughness * roughness;
    float a2 = a * a;
    float NH = max(dot(normal,halfway_direction), 0.0);
    float NH2 = NH * NH;

    float numerator = a2;
    float denominator = (NH2 * (a2 - 1.0) + 1.0);
    denominator = PI * denominator * denominator;

    return numerator / denominator;
}

float GGX_Schlick(float NV, float roughness) {
    float r = (roughness + 1.0);
    float k = (r * r) / 8.0;

    float nom = NV;
    float denom = NV * (1.0 - k) + k;

    return nom / denom;
}

float geometry_smith(vec3 normal, vec3 view_position, vec3 light_direction, float roughness)
{
    float NV = max(dot(normal,view_position),0.0);
    float NL = max(dot(normal,light_direction), 0.0);
    float ggx2 = GGX_Schlick(NV, roughness);
    float ggx1 = GGX_Schlick(NL, roughness);

    return ggx1 * ggx2;
}

vec3 fresnel_schlick(float cos_theta, vec3 f) {
    return f + (1 - f) * pow(1.0 - cos_theta, 5.0);
}

void main() {
    vec3 f_normal = normalize(v_normal);
    vec3 view_direction = normalize(view_position - frag_pos);
    vec3 f_color = vec3(0.0);

    for(int i = 0; i < lights_count; i++) {
        vec3 light_direction = light_direction(light[i], frag_pos);
        vec3 halfway_direction = normalize( view_position + light_direction);

        vec3 f = mix(vec3(0.04),material.albedo, material.metallic);

        float attenuation = attenuation(light[i], frag_pos);
        vec3 radiance = light_color(light[i], light_direction) * attenuation;

        float NDF = GGX_distribution(f_normal, halfway_direction, material.roughness);
        float G = geometry_smith(f_normal, view_position, light_direction, material.roughness);
        vec3 F = fresnel_schlick(max(dot(halfway_direction, view_direction), 0.0), f);

        vec3 kd = vec3(1.0) - F;
        kd *= 1.0 - material.metallic;

        vec3 nominator = NDF * G * F;
        float denominator = 4 * max(dot(f_normal,view_position), 0.0) * max(dot(f_normal, light_direction), 0.0) + 0.001;
        vec3 specular = nominator / denominator;
    
        float NL = max(dot(f_normal, light_direction), 0.0);
        vec3 lo = (kd * material.albedo / PI + specular) * radiance * NL;

        vec3 ambient = vec3(0.03) * material.albedo * material.ao;
        vec3 color = ambient + lo;
        color = color / (color + vec3(1.0));
        color = pow(color, vec3(1.0 / 2.2));
        f_color = f_color + color;
    }
    color_out = vec4(f_color,1.0);
}