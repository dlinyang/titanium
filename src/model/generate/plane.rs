use crate::base::mesh::Mesh;
use rmu::vector::Vector3;

pub fn plane(length: f32) -> Mesh {
    let half_l = length / 2.0;

    let v1 = Vector3::new( half_l, -half_l, 0.0);
    let v2 = Vector3::new( half_l,  half_l, 0.0);
    let v3 = Vector3::new(-half_l,  half_l, 0.0);
    let v4 = Vector3::new(-half_l, -half_l, 0.0);

    let up = Vector3::new(0.0, 0.0, 1.0);
    
    Mesh {
        vertices: vec![v1,v2,v3,v4],
        vertex_normals: vec![up],
        faces: vec![vec![[1,1,0], [2,1,0], [3,1,0], [4,1,0]]],
        uv: Vec::new(),
    }
} 