use crate::base::Position;
use glium::Display;
use glium::{VertexBuffer, IndexBuffer, Program};

pub struct FrameData {
    pub vertex_buffer: VertexBuffer<Position>,
    pub index_buffer: IndexBuffer<u32>,
    pub shadow_map: Program,
}

use crate::renderer::pipeline::shader::glsl::*;

impl FrameData {
    pub fn new(display: &Display) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(
            display,
            &[Position::new([-1.0, -1.0], [0.0, 0.0]),
              Position::new([-1.0,  1.0], [0.0, 1.0]), 
              Position::new([ 1.0,  1.0], [1.0, 1.0]),
              Position::new([ 1.0, -1.0], [1.0, 0.0])]
        ).unwrap();

        let index_buffer = IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[0,1,2,0,2,3]).unwrap();

        let version = glsl_version(4, 60);
        let shadow_map_vert = glsl(version.clone(),vert_lib(),shadow_map_vert());
        let shadow_map_frag = glsl(version, String::new(), shadow_map_frag());

        let shadow_map = Program::from_source(display, shadow_map_vert.as_str(), shadow_map_frag.as_str(), None).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            shadow_map,
        }
    }
}