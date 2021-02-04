use rmu::vector::Vector3;
use crate::base::mesh::Mesh;

pub fn grid(step: f32, row: usize, column: usize) -> Mesh {
    let w = step * row as f32;
    let h = step * row as f32;
    let  x = -w / 2.0;
    let  y = -h / 2.0;

    let mut vertices: Vec<Vector3> = Vec::new();

    for i in 0..(row - 1) {
        vertices.push(Vector3::new(x + step * i as f32, y ,0.0));
        vertices.push(Vector3::new(x + step * i as f32, y + h, 0.0));
    }

    for i in 0..(column - 1) {
        vertices.push(Vector3::new(x, y + step * i as f32, 0.0));
        vertices.push(Vector3::new(x + w, y + step * i as f32, 0.0));
    }

    let mut i = 0;
    let mut edges = Vec::new();
    while  i < column * row - 1 {
        edges.push([i as u32, (i+1) as u32]);
        i = i + 2;
    }

    Mesh {
        vertices,
        vertex_normals: Vec::new(),
        uv: Vec::new(),
        edges: Vec::new(),
        faces: Vec::new(),
    }

}