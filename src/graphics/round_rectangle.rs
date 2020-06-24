use rmu::raw::{Vec2f, Vec4f};
use crate::base::Position;
use crate::renderer::canvas::graphics::{Graphics, GraphicsType};

#[derive(Clone,Copy)]
pub struct RoundRectangle {
    pub anchor: Vec2f,
    pub w: f32,
    pub h: f32,
    pub radius: f32,
    pub color: Vec4f,
    pub graphics_type: GraphicsType,
}

impl RoundRectangle {
    pub fn new() -> Self {
        RoundRectangle {
            anchor: [0.9,0.9],
            w: 0.1,
            h: 0.05,
            radius: 0.01,
            color: [0.1, 0.1, 0.5, 1.0],
            graphics_type: GraphicsType::Polygon,
        }
    }
}