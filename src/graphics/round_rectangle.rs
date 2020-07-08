use rmu::raw::{Vec2f, Vec4f};
use crate::base::Position;
use crate::renderer::canvas::graphics::{Graphics, GraphicsType};

#[derive(Clone,Copy)]
pub struct RoundRectangle {
    pub anchor: Vec2f,
    pub w: f32,
    pub h: f32,
    pub round_angle: RoundAngle,
    pub color: Vec4f,
    pub graphics_type: GraphicsType,
}

impl RoundRectangle {
    pub fn new() -> Self {
        RoundRectangle {
            anchor: [0.9,0.9],
            w: 0.1,
            h: 0.05,
            round_angle: Default::default(),
            color: [0.1, 0.1, 0.5, 1.0],
            graphics_type: GraphicsType::PolygonFill,
        }
    }
}

#[derive(Copy,Clone)]
pub struct RoundAngle {
    pub top_left: Option<f32>,
    pub top_right: Option<f32>,
    pub bottom_right: Option<f32>,
    pub bottom_left: Option<f32>,
}

impl Default for RoundAngle {
    fn default() -> Self {
        Self {
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }
}