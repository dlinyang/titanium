struct BlinnPhongBRDF {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

uniform BlinnPhongBRDF material;

in vec3 v_normal;
in vec3 frag_pos;

out vec4 color_out;

void main() {

    vec3 f_normal = normalize(v_normal);
    vec3 view_direction = normalize(view_position - frag_pos);
    vec3 f_color = vec3(0.0);

    for(int i = 0; i < lights_count; i++) {
        vec3 light_direction = normalize(light_direction(light[i], frag_pos));
        vec3 color = light_color(light[i], light_direction);
        vec3 halfway_direction = normalize(light_direction +  view_direction);

        float attenuation =  attenuation(light[i], frag_pos);

        vec3 ambient = color * material.ambient;
        vec3 specular = color * pow(max(dot(f_normal, halfway_direction), 0.0), material.shininess);
        vec3 diffuse = color * max(dot(f_normal, light_direction),0.0);
        float visibility = visibility(shadow[i].mvp * vec4(frag_pos,1.0),shadow[i].tex,0.0);
        f_color = f_color + (ambient + visibility * (diffuse + specular)) * attenuation;
    }

    color_out = hdr(vec4(f_color,1.0));
}