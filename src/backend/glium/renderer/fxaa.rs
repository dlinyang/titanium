use crate::base::Position;
use glium::Display;
use glium::{VertexBuffer, IndexBuffer, Program};
use glium::texture::Texture2d;

pub struct Fxaa {
    pub vertex_buffer: VertexBuffer<Position>,
    pub index_buffer: IndexBuffer<u32>,
    pub fxaa: Program, 
    pub render_buffer: Texture2d,
    pub depth_buffer: glium::framebuffer::DepthRenderBuffer,
}

use  crate::renderer::pipeline::shader::glsl::*;

impl Fxaa {
    pub fn new(display: &Display) -> Self {
        let (w,h) = display.gl_window().window().inner_size().into();

        let vertex_buffer = glium::VertexBuffer::new(
            display,
            &[Position::new([-1.0, -1.0], [0.0, 0.0]),
              Position::new([-1.0,  1.0], [0.0, 1.0]), 
              Position::new([ 1.0,  1.0], [1.0, 1.0]),
              Position::new([ 1.0, -1.0], [1.0, 0.0])]
        ).unwrap();

        let index_buffer = IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[0,1,2,0,2,3]).unwrap();

        let version = glsl_version(4, 60);
        let canvas_vert = glsl(version.clone(), String::new() , canvas_vert());
        let fxaa_code = glsl(version.clone(), String::new(), fxaa());
        let fxaa = Program::from_source(
            display, canvas_vert.as_str(), fxaa_code.as_str(), None
        ).unwrap();

        let render_buffer = Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            w,
            h,
        ).unwrap();

        Self {
            vertex_buffer,
            index_buffer,
            fxaa,
            render_buffer,
            depth_buffer: glium::framebuffer::DepthRenderBuffer::new(display, glium::texture::DepthFormat::F32, w, h).unwrap(),
        }
    }
}