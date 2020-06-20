use rmu::raw::Vec3f;

//
#[derive(Copy,Clone)]
pub enum Light {
    PointLight(PointLight),
    ParallelLight(ParallelLight),
    SpotLight(SpotLight),
}

//
#[derive(Copy,Clone)]
pub struct PointLight{
    pub position: Vec3f,
    pub color: Vec3f,
}

impl PointLight {
    pub fn new(position: Vec3f, color: Vec3f) -> Self {
        PointLight {
            position,
            color,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ParallelLight {
    pub direction: Vec3f,
    pub color: Vec3f,
}

impl ParallelLight {
    pub fn new(direction: Vec3f,color: Vec3f) -> Self {
        Self {
            direction,
            color,
        }
    }
}

#[derive(Copy,Clone)]
pub struct SpotLight {
    pub position: Vec3f,
    pub direction: Vec3f,
    pub theta: f32,
    pub color: Vec3f,
}

impl SpotLight {
    pub fn new(position: Vec3f, direction: Vec3f, theta: f32, color: Vec3f) -> Self {
        Self {
            position,
            direction,
            theta,
            color,
        }
    }
}