use rmu::raw::{Vec2f,Vec4f}; 
use crate::base::Position;
use crate::base::material::color_canvas;
use crate::renderer::canvas::graphics::{Graphics, GraphicsType};

#[derive(Clone)]
pub struct Points {
    pub point_array: Vec<Vec2f>,
    pub color: Vec4f,
}

impl Points {
    pub fn new() -> Self {
        Self {
            point_array: vec![[0.0,0.0]],
            color: [0.5,0.5,0.5,1.0],
        }
    }

    pub fn create(point_array: Vec<Vec2f>, color: Vec4f) -> Self {
        Self {
            point_array,
            color,
        }
    }

    pub fn positions(&self) -> Vec<Position> {
        self.point_array.iter().map(|x| Position::from(*x)).collect()
    }
}

impl From<Points> for Graphics {
    fn from(points: Points) -> Graphics {
        Graphics {
            positions: points.positions(),
            material: color_canvas(points.color),
            graphics_type: GraphicsType::Points,
        }
    }
}