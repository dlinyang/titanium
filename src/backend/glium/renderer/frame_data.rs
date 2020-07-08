use crate::base::Position;
use glium::Display;
use glium::{VertexBuffer, IndexBuffer};

pub struct FrameData {
    pub vertex_buffer: VertexBuffer<Position>,
    pub index_buffer: IndexBuffer<u32>,
}

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

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}