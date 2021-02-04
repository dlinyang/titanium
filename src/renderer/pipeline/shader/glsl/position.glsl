in layout(location = 0) vec2 position;
in layout(location = 1) vec2 tex_coordinate;

out vec2 v_tex_coordinate;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_tex_coordinate = tex_coordinate;
}