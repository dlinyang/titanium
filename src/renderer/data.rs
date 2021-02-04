use crate::base::Vertex;
use crate::base::Indices;
use rmu::raw::Mat4f;

#[derive(Copy,Clone,PartialEq)]
pub enum DataUpdate {
    ALL,
    Material,
    Transfrom,
    Statue,
    Not,
}

#[derive(Clone)]
pub struct Object {
    pub mesh_name: String,
    pub material_name: String,
    pub transform: Mat4f,
    pub update: DataUpdate,
}

impl Object {
    #[inline]
    pub fn new(mesh_name: String, material_name: String, transform: Mat4f) -> Self {
        Self {
            mesh_name,
            material_name,
            transform,
            update: DataUpdate::ALL,
        }
    }
}

pub struct GMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Indices,
}

impl GMesh {
    #[inline]
    pub fn new(vertices: Vec<Vertex>, indices: Indices) -> Self {
        Self {
            vertices,
            indices,
        }
    }
}