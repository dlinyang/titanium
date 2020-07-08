uniform sampler2D canvas;

in vec2 v_tex_coordinate;

out vec4 color_out;

void main() {
    color_out = texture(canvas, v_tex_coordinate);
}