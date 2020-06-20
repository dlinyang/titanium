struct PureColor {
    vec3 color;
};

uniform PureColor material;

out vec4 color_out;

void main() {
    color_out = vec4(material.color,1.0);
}