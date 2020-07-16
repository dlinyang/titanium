/// # Data Buffer
use rmu::raw::{Vec4f,Mat4f};
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::texture::texture2d::Texture2d;
use glium::texture::depth_texture2d::DepthTexture2d;
use std::collections::HashMap;
use crate::base::{Vertex, material::Material, camera::Camera};
use crate::renderer::Light;
use glium::Display;
use std::rc::Rc;

pub struct DataBuffer {
    pub scene_data    : Rc<SceneBuffer>,
    pub light_buffer  : Rc<LightBuffer>,
    pub texture_buffer: HashMap<String,Texture2d>,
    pub depth_texture : Option<DepthTexture2d>,
    pub camera        : Camera,
    pub bg_color      : Vec4f,
}

impl DataBuffer {
    pub fn new(display: &Display) -> Self {
        Self {
            scene_data: Rc::new(Default::default()),
            light_buffer: Rc::new(LightBuffer::new(display)),
            texture_buffer: HashMap::new(),
            depth_texture: None,
            camera: Default::default(),
            bg_color: [1.0,1.0,1.0,1.0],
        }
    }
}

use glium::buffer::*;
use glium::implement_uniform_block;

implement_uniform_block!(Light,color_flux,position,direction_type,cut_off,outer_cut_off,linear,quadratic);

pub struct LightBuffer {
    pub lights: HashMap<String,Light>,
    pub shadow_maps: HashMap<String,DepthTexture2d>,
    pub shadow_map_views: Vec<Mat4f>,
    pub shadow_map_size: u32,
    pub buffer: Buffer<[Light]>,
}

impl LightBuffer {
    pub fn new(display: &Display) -> Self {
        Self {
            lights: HashMap::new(),
            shadow_maps: HashMap::new(),
            shadow_map_views: Vec::new(),
            shadow_map_size: 512,
            buffer: Buffer::new(display, vec![Light::new()].as_slice(), BufferType::UniformBuffer, BufferMode::default()).unwrap(),
        }
    }

    pub fn unifrom_buffer(&self) -> &Buffer<[Light]> {
        &self.buffer
    }

    pub fn light_number(&self) -> usize{
        self.lights.len()
    }
}

pub struct SceneBuffer {
    pub data: HashMap<String,SceneData>,
    pub same_material_data: HashMap<String,HashMap<String,()>>,
}

impl Default for SceneBuffer {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            same_material_data: HashMap::new(),
        }
    }
}

pub struct SceneData {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u32>,
    pub material: Material,
    pub transform: Mat4f,
}

impl SceneData{
    pub fn new(
        vertex_buffer: VertexBuffer<Vertex>,
        indices: IndexBuffer<u32>,
        material: Material,
        transform: [[f32; 4]; 4],
    ) -> Self {
        Self {
            vertex_buffer,
            indices,
            material,
            transform,
        }
    }
}