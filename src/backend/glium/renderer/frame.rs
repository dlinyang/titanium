use crate::base::Position;
use glium::Display;
use glium::{VertexBuffer, IndexBuffer, Program};

pub struct Frame {
    pub vertex_buffer: VertexBuffer<Position>,
    pub index_buffer: IndexBuffer<u32>,
    pub program: Program, 
}

use  crate::renderer::pipeline::shader::glsl::*;

impl Frame {
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
        let vertex_shader = glsl(version.clone(), String::new() , canvas_vert());
        let fragment_shader = glsl(version.clone(), String::new(), frame());
        let program = Program::from_source(
            display, vertex_shader.as_str(), fragment_shader.as_str(), None
        ).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            program,
        }
    }
}