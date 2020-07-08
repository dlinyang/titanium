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
        let version = glsl_version(4, 50);

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

        // canvas render pass need turn off depth write and depth test 
        self.shaders.insert("Color Canvas".into(), RenderPass::pass(color_canvas, None).with_depth(ZTest::Always, false));
        self.shaders.insert("Image Canvas".into(), RenderPass::pass(image_canvas, None).with_depth(ZTest::Always, false));
        self.shaders.insert("Font Canvas".into(), RenderPass::pass(font_canvas, None).with_depth(ZTest::Always, false));
        //
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