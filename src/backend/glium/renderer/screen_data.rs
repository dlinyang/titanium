use crate::base::ImagePosition;
use glium::Display;
use glium::{VertexBuffer, IndexBuffer, Program};

pub struct ScreenData {
    pub vertex_buffer: VertexBuffer<ImagePosition>,
    pub index_buffer: IndexBuffer<u32>,
    pub shadow_map: Program,
    pub color: Program,
    pub image: Program,
    pub font: Program,
}

use crate::renderer::pipeline::shader::glsl::*;

impl ScreenData {
    pub fn new(display: &Display) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(
            display,
            &[ImagePosition::new([-1.0, -1.0], [0.0, 0.0]),
              ImagePosition::new([-1.0,  1.0], [0.0, 1.0]), 
              ImagePosition::new([ 1.0,  1.0], [1.0, 1.0]),
              ImagePosition::new([ 1.0, -1.0], [1.0, 0.0])]
        ).unwrap();

        let index_buffer = IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[0,1,2,0,2,3]).unwrap();

        let version = glsl_version(4, 60);

        let position = glsl(version.clone(), String::new(), position());
        let image_position = glsl(version.clone(), String::new(), image_position());
        let color_code = glsl(version.clone(), String::new(), color());
        let image_code = glsl(version.clone(), String::new(), image());
        let font_code = glsl(version.clone(), String::new(), font());

        let shadow_map_vert = glsl(version.clone(),vert_lib(),shadow_map_vert());
        let shadow_map_frag = glsl(version, String::new(), shadow_map_frag());

        let shadow_map = Program::from_source(
            display, shadow_map_vert.as_str(), shadow_map_frag.as_str(), None
        ).unwrap();

        let color = Program::from_source(
            display, position.as_str(), color_code.as_str(), None
        ).unwrap();

        let image = Program::from_source(
            display, image_position.as_str(), image_code.as_str(), None
        ).unwrap();

        let font = Program::from_source(
            display, image_position.as_str(), font_code.as_str(), None
        ).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            shadow_map,
            color,
            font,
            image,
        }
    }
}