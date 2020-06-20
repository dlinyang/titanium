use rmu::raw::{Vec2f, Vec4f};
use crate::base::Position;
use crate::base::material::color_canvas;
use crate::renderer::canvas::graphics::{Graphics, GraphicsType};

#[derive(Clone,Copy)]
pub struct Circle {
    ///anchor is center point
    pub anchor: Vec2f,
    pub radius: f32,
    pub color: Vec4f,
    pub graphics_type: GraphicsType
}

impl Circle {
    pub fn new() -> Self {
        Self {
            anchor: [0.9, 0.9],
            radius: 0.1,
            color: [0.1,0.1,0.5,1.0],
            graphics_type: GraphicsType::Polygon,
        }
    }

    pub fn create(anchor: Vec2f, radius: f32, color: Vec4f, graphics_type: GraphicsType) -> Self {
        Self {
            anchor,
            radius,
            color,
            graphics_type,
        }
    }
}

impl Into<Graphics> for Circle {
    fn into(self) -> Graphics {
        
        let basic_sampling_count: f32 = 16.0 + self.radius * 100.0;

        Graphics{
            positions: circle(self.radius, self.anchor, basic_sampling_count),
            material: color_canvas(self.color),
            graphics_type: self.graphics_type,
        }
    }
}

use std::f32::{consts::PI};
// get the point of angle in a position
fn circle(radius: f32 ,position: Vec2f, sampling_count: f32 ) -> Vec<Position> {

    let [x,y] = position;
    let delta = PI * 2.0 / (sampling_count as f32);
    let mut pos: Vec<Position> = Vec::new();
    
    let mut i: f32 = 0.0;
    while i < 2.0 * PI {

        pos.push(Position::from([x + i.cos() * radius, y + i.sin() * radius]));

        i = i + delta;
    }
    pos
}