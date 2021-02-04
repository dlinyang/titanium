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

pub fn position() -> String {
    String::from(include_str!("glsl/position.glsl"))
}

pub fn font() -> String {
    String::from(include_str!("glsl/font.glsl"))
}

pub fn color() -> String {
    String::from(include_str!("glsl/color.glsl"))
}

pub fn image() -> String {
    String::from(include_str!("glsl/image.glsl"))
}

pub fn scene() -> String {
    String::from(include_str!("glsl/scene.glsl"))
}

pub fn shadow_map_vert() -> String {
    String::from(include_str!("glsl/shadow_map_vert.glsl"))
}

pub fn shadow_map_frag() -> String {
    String::from(include_str!("glsl/shadow_map_frag.glsl"))
}