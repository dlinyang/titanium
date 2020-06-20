use glium::program::{Program, Binary};
use glium::Display;

use crate::renderer::pipeline::ShaderBuffer;
use crate::renderer::pipeline::shader::glsl::*;

use std::collections::HashMap;


pub struct GLShaderBuffer {
    pub shaders: HashMap<String,Program>,
}

impl GLShaderBuffer {
    pub fn new(display: &Display) -> Self {
        let version = glsl_version(4, 60);

        let canvas_vert = glsl(version.clone(), String::new(), canvas_vert());
        let canvas_code = glsl(version.clone(), String::new(), color_canvas());
        let canvas_with_texture_code = glsl(version.clone(), String::new(), image_canvas());
        let font_code = glsl(version.clone(), String::new(), font_canvas());

        let color_canvas = Program::from_source(
            display, canvas_vert.as_str(), canvas_code.as_str(), None
        ).unwrap();

        let image_canvas = Program::from_source(
            display, canvas_vert.as_str(), canvas_with_texture_code.as_str(), None
        ).unwrap();

        let font_canvas = Program::from_source(
            display, canvas_vert.as_str(), font_code.as_str(), None
        ).unwrap();

        let vertex_shader = glsl(version.clone(), vert_lib(), base_vert());
        let pure_color_code = glsl(version.clone(), String::new(), pure_color());
        let blinn_phong_brdf_code = glsl(version.clone(), light_lib(10), blinn_phong_brdf());
        let cook_torrance_brdf_code = glsl(version.clone(), light_lib(10), cook_torrance_brdf());
        
        let blinn_phong_brdf = Program::from_source(
            display, vertex_shader.as_str(), blinn_phong_brdf_code.as_str(), None
        ).unwrap();

        let cook_torrance_brdf = Program::from_source(
            display, vertex_shader.as_str(), cook_torrance_brdf_code.as_str(), None
        ).unwrap();
        
        let pure_color = Program::from_source(
            display, vertex_shader.as_str(), pure_color_code.as_str(), None
        ).unwrap();
        
        let mut shaders = HashMap::new();

        shaders.insert("Color Canvas".into(), color_canvas);
        shaders.insert("Image Canvas".into(), image_canvas);
        shaders.insert("Font Canvas".into(), font_canvas);
        shaders.insert("Blinn Phong BRDF".into(), blinn_phong_brdf);
        shaders.insert("Cook Torrance BRDF".into(), cook_torrance_brdf);
        shaders.insert("Pure Color Material".into(), pure_color);

        GLShaderBuffer {
            shaders,
        }
    }
}

impl ShaderBuffer<Program> for GLShaderBuffer {

    fn shader(&self, shader_name: &String) -> Option<&Program> {
        self.shaders.get(shader_name)
    }
}