///
/// 
/// 
/// 
/// 
use rmu::raw::{Vec3f,Vec4f,Mat4f,ID4F};
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::texture::texture2d::Texture2d;
use glium::texture::depth_texture2d::DepthTexture2d;
use std::collections::HashMap;
use crate::base::{Vertex, Position, material::Material};
use crate::renderer::Light;
use glium::Display;
use std::rc::Rc;

pub struct DataBuffer {
    pub scene_data    : Rc<SceneBuffer>,
    pub canvas_data   : Rc<CanvasBuffer>,
    pub light_buffer  : LightBuffer,
    pub texture_buffer: HashMap<String,Texture2d>,
    pub depth_texture : Option<DepthTexture2d>,
    pub view          : Mat4f,
    pub view_position : Vec3f,
    pub project       : Mat4f,
    pub bg_color      : Vec4f,
}

impl DataBuffer {
    pub fn lights(&mut self,display: &Display) -> Buffer<[Light]> {
        if let None = self.light_buffer.buffer {
            let lights: Vec<Light> = self.light_buffer.lights.values().map(|x| {*x}).collect();
            Buffer::new(display, lights.as_slice(), BufferType::UniformBuffer, BufferMode::default()).unwrap()
        } else {
            self.light_buffer.buffer.take().unwrap()
        }
    }

    pub fn light_number(&self) -> usize{
        self.light_buffer.lights.len()
    }
}

impl Default for DataBuffer {
    fn default() -> Self {
        Self {
            scene_data: Rc::new(Default::default()),
            canvas_data: Rc::new(Default::default()),
            light_buffer: LightBuffer::new(),
            texture_buffer: HashMap::new(),
            depth_texture: None,
            view: ID4F,
            view_position: Default::default(),
            project: ID4F,
            bg_color: [1.0,1.0,1.0,1.0],
        }
    }
}

use glium::buffer::*;
use glium::implement_uniform_block;

implement_uniform_block!(Light,color_flux,position,direction_type,cut_off,outer_cut_off,linear,quadratic);

pub struct LightBuffer {
    pub lights: HashMap<String,Light>,
    pub buffer: Option<Buffer<[Light]>>,
}

impl LightBuffer {
    pub fn new() -> Self {
        Self {
            lights: HashMap::new(),
            buffer: None,
        }
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

pub struct CanvasBuffer {
    pub id: u64, 
    pub data: Vec<RenderLayer>,
}

impl CanvasBuffer {
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn update(&mut self, render_layer: RenderLayer) {

        let mut i: usize = 0;

        while i < self.data.len() {
            if render_layer.id == self.data[i].id {
                break;
            } else  {
                i = i + 1;
            }
        }

        if i == self.data.len() {
            self.data.push(render_layer);
        } else {
            self.data[i] = render_layer;
        }
    }
}

impl Default for CanvasBuffer {
    fn default() -> Self {
        Self {
            id: 0,
            data: Vec::new(),
        }
    }
}

pub struct RenderLayer {
    pub id: u64,
    pub graphics: Option<GraphicsData>,
    pub text: Option<TextData>,
}

impl RenderLayer {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            graphics: None,
            text: None,
        }
    }

    pub fn set_text(&mut self, text_data: TextData)  {
        self.text = Some(text_data);
    }

    pub fn set_graphics(&mut self, graphics_data: GraphicsData) {
        self.graphics = Some(graphics_data);
    }
}

pub struct TextData {
    pub vertex_buffer: VertexBuffer<Position>,
    pub texture: Texture2d,
    pub indices: IndexBuffer<u32>,
    pub material: Material,
}

use glium::index::NoIndices;

pub struct GraphicsData {
    pub vertex_buffer: VertexBuffer<Position>,
    pub indices: NoIndices,
    pub material: Material,
}