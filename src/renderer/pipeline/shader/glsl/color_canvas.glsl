uniform vec4 color;

in vec2 v_tex_coordinate;
              
out vec4 f_color;
                       
void main() {
    f_color = color;
}