pub fn glsl(version: String, include: String, source_code: String) -> String {
    format!("{}\n{}\n{}", version, include, source_code)
}

pub fn glsl_version(major_version: u8, minior_version: u8) -> String {
    format!("#version {}{}\n\n", major_version, minior_version)
}

pub fn vert_lib() -> String {
    String::from(include_str!("glsl/vert_lib.glsl"))
}

pub fn light_lib(light_count: u16) -> String {
    String::from(
        format!("#define LIGHTS_MAX_NUMBER {}\n{}",
                light_count,
                include_str!("glsl/light_lib.glsl")
        )
    ) 
}

pub fn base_vert() -> String {
    String::from(include_str!("glsl/base_vert.glsl"))
}

pub fn blinn_phong_brdf() -> String {
    String::from(include_str!("glsl/blinn_phong_brdf.glsl"))
}

pub fn cook_torrance_brdf() -> String {
    String::from(include_str!("glsl/cook_torrance_brdf.glsl"))
}

pub fn pure_color() -> String {
    String::from(include_str!("glsl/pure_color.glsl"))
}

pub fn canvas_vert() -> String {
    String::from(include_str!("glsl/canvas_vert.glsl"))
}

pub fn font_canvas() -> String {
    String::from(include_str!("glsl/font_canvas.glsl"))
}

pub fn color_canvas() -> String {
    String::from(include_str!("glsl/color_canvas.glsl"))
}

pub fn image_canvas() -> String {
    String::from(include_str!("glsl/image_canvas.glsl"))
}