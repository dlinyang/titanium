use rmu::raw::{Vec2f, Vec3f};
use rmu::vector::{Vector3,Vector2};

#[derive(Debug,Copy, Clone)]
pub struct Vertex {
    pub position: Vec3f,
    pub normal: Vec3f,
    pub tex_coordinate: Vec2f,
}

impl Vertex {
    #[inline]
    pub fn new(position: Vec3f, normal: Vec3f, tex_coordinate: Vec2f) -> Self {
        Vertex {
            position,
            normal,
            tex_coordinate,
        }
    }
}

impl From<Vec3f> for Vertex {
    #[inline]
    fn from(position: Vec3f) -> Self {
        Self::new(position, [0.0, 0.0, 0.0], [0.0, 0.0])
    }
}

impl From<Vector3> for Vertex {
    #[inline]
    fn from(position: Vector3) -> Self {
        Self::new(position.into(), [0.0, 0.0, 0.0], [0.0, 0.0])
    }
}

//use for 2d and ui
#[derive(Copy,Clone)]
pub struct Position {
    pub position: Vec2f,
    pub tex_coordinate: Vec2f,
}

impl Position {
    #[inline]
    pub fn new(position: Vec2f, tex_coordinate: Vec2f) -> Self {
        Position {
            position,
            tex_coordinate,
        }
    }
}

impl From<Vec2f> for Position {
    fn from(position: Vec2f) -> Self {
        Self::new(position, [0.0, 0.0])
    }
}

impl From<Vector2> for Position {
    fn from(position: Vector2) -> Self {
        Self::new(position.into(), [0.0, 0.0])
    }
}