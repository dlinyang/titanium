in layout(location = 0) vec3 position;
in layout(location = 1) vec3 normal;
in layout(location = 2) vec3 tex_coordinate;

out vec3 frag_pos;
out vec3 v_normal;

void main() {
    vec4 pos = transform * vec4(position,1.0);
    gl_Position = project * view * pos;
    
    v_normal = normal;
    frag_pos = vec3(pos);
}