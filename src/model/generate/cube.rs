use crate::base::mesh::Mesh;
use rmu::vector::Vector3;

pub fn cube(length: f32) -> Mesh {
    let half_l = length / 2.0;

    let v1 = Vector3::new(half_l, -half_l, half_l);
    let v2 = Vector3::new(half_l, half_l, half_l);
    let v3 = Vector3::new(half_l, half_l, -half_l);
    let v4 = Vector3::new(half_l, -half_l, -half_l);

    let v5 = Vector3::new(-half_l, -half_l, half_l);
    let v6 = Vector3::new(-half_l, half_l, half_l);
    let v7 = Vector3::new(-half_l, half_l, -half_l);
    let v8 = Vector3::new(-half_l, -half_l, -half_l);

    let front = Vector3::new(1.0, 0.0, 0.0);
    let back = Vector3::new(-1.0, 0.0, 0.0);
    let right = Vector3::new(0.0, 1.0, 0.0);
    let left = Vector3::new(0.0, -1.0, 0.0);
    let top = Vector3::new(0.0, 0.0, 1.0);
    let bottom = Vector3::new(0.0, 0.0, -1.0);
    
    Mesh {
        vertices: vec![v1,v2,v3,v4,v5,v6,v7,v8],
        vertex_normals: vec![front, back, top, right, bottom, left],
        faces: vec![vec![[1,1,0], [2,1,0], [3,1,0], [4,1,0]],
                    vec![[5,2,0], [6,2,0], [7,2,0], [8,2,0]], 
                    vec![[1,3,0], [2,3,0], [6,3,0], [5,3,0]], 
                    vec![[2,4,0], [3,4,0], [7,4,0], [6,4,0]], 
                    vec![[3,5,0], [4,6,0], [8,5,0], [7,5,0]], 
                    vec![[4,6,0], [1,6,0], [5,6,0], [8,6,0]]],
        uv: Vec::new(),
    }
} 