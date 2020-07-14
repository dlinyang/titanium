in layout(location = 0) vec3 position;

void main() {
    gl_Position = project * view * transform * vec4(position,1.0);
}