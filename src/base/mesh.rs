use rmu::vector::{Vector3,Vector2};

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub vertex_normals: Vec<Vector3>,
    /// x is vertex ,index y is normal index, z is uv index 
    /// index is start from 1, if index is 0 , it' mean no attribute
    pub faces: Vec<Vec<[u32;3]>>,
    pub uv: Vec<Vector2>,
}