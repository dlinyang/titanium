#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    a_pos:[f32; 2],
    a_uv: [f32; 2],
}

pub struct Vector2 {
    x : i32,
    y : i32,
}
pub struct Vector3 {
    x : i32,
    y : i32,
    z : i32,
} 

pub struct Polygon{
    points : Vec<Vector3>
}