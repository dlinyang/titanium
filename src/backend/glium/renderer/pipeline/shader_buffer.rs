use glium::program::{Program, Binary};
use glium::Display;

use crate::renderer::pipeline::*;
use crate::renderer::pipeline::shader::glsl::*;

use std::collections::HashMap;


pub struct GLShaderBuffer {
    pub shaders: HashMap<String,RenderPass<Program>>,
}

impl GLShaderBuffer {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }

    pub fn load_bulidin(&mut self,display: &Display) {
        let version = glsl_version(4, 60);

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
        
        self.shaders.insert("Blinn Phong BRDF".into(), RenderPass::pass(blinn_phong_brdf, None));
        self.shaders.insert("Cook Torrance BRDF".into(), RenderPass::pass(cook_torrance_brdf, None));
        self.shaders.insert("Pure Color Material".into(), RenderPass::pass(pure_color, None));
    }
}

impl ShaderBuffer<Program> for GLShaderBuffer {

    fn shader(&self, shader_name: &String) -> Option<&RenderPass<Program>> {
        self.shaders.get(shader_name)
    }
}