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

    pub fn to_graphics(&self) -> Graphics {
        Graphics {
            positions: self.positions(),
            material: color_canvas(self.color),
            graphics_type: GraphicsType::Points,
        }
    }

    pub fn positions(&self) -> Vec<Position> {

        let mut result = Vec::new();

        for x in &self.point_array {
            result.push(Position::from(*x));
        }
        result
    }

}