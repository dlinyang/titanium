use crate::base::Vertex;
use crate::base::Index;
use crate::base::material::Material;
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
pub struct RenderData {
    pub vertices: Vec<Vertex>,
    pub indices: Index,
    pub material: Material,
    pub transform: Mat4f,
    pub update: DataUpdate,
}

impl RenderData {
    #[inline]
    pub fn new(vertices: Vec<Vertex>, indices: Index, material: Material, transform: Mat4f) -> Self {
        Self {
            vertices,
            indices,
            material,
            transform,
            update: DataUpdate::ALL,
        }
    }
}