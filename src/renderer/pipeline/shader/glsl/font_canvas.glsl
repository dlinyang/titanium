uniform sampler2D font_tex;
uniform vec4 color;

in vec2 v_tex_coordinate;

out vec4 f_color;

void main() {
    f_color = color * vec4(1.0,1.0,1.0, texture(font_tex, v_tex_coordinate).r);
}