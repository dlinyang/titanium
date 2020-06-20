///
/// 
/// 
/// 
/// 
use rmu::raw::{Vec3f,Vec4f,Mat4f,ID4F};
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::texture::texture2d::Texture2d;
use std::collections::HashMap;
use crate::base::{Vertex, Position, material::Material};
use crate::renderer::Light;

pub struct DataBuffer {
    pub scene_data: SceneBuffer,
    pub canvas_data: CanvasBuffer,
    pub lights: HashMap<String,Light>,
    pub view: Mat4f,
    pub view_position: Vec3f,
    pub project: Mat4f,
    pub bg_color: Vec4f,
}

impl DataBuffer {
    pub fn lights(&self) -> Vec<Light> {
        self.lights.values().map(|v| v.clone()).collect()
    }
}

impl Default for DataBuffer {
    fn default() -> Self {
        Self {
            scene_data: Default::default(),
            canvas_data: Default::default(),
            lights: HashMap::new(),
            view: ID4F,
            view_position: Default::default(),
            project: ID4F,
            bg_color: [1.0,1.0,1.0,1.0],
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
    pub gragphics: Option<GraphicsData>,
    pub text: Option<TextData>,
}

impl RenderLayer {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            gragphics: None,
            text: None,
        }
    }

    pub fn set_text(&mut self, text_data: TextData)  {
        self.text = Some(text_data);
    }

    pub fn set_graphics(&mut self, graphics_data: GraphicsData) {
        self.gragphics = Some(graphics_data);
    }
}

pub struct TextData {
    pub vertex_buffer: VertexBuffer<Position>,
    pub texture: Texture2d,
    pub indices: IndexBuffer<u32>,
    pub material: Material,
}

pub struct GraphicsData {
    pub vertex_buffer: VertexBuffer<Position>,
    pub indices: IndexBuffer<u32>,
    pub material: Material,
}