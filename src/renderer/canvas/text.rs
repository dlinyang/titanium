use crate::base::utils::Size;
use crate::base::material::*;

use rmu::raw::{Vec2f,Vec4f};
//

#[derive(Clone)]
pub struct Text {
    pub context: String,
    pub font: String,
    pub position: Vec2f,
    pub size: Size,
    pub width: f32,
    pub material: Material,
}

impl Text {
    pub fn new(context: String) -> Self {
        Self {
            context,
            font: String::default(),
            position: [0.0,0.0],
            size: Size {width: 1.0, height:1.0},
            width: 10.0,
            material: font_canvas([0.0,0.0,0.0,1.0]),
        }
    }

    pub fn create(context: String, font: String ,position:Vec2f, size: Size, width: f32, color: Vec4f) -> Self{
        Self {
            context,
            font,
            position,
            size,
            width,
            material: font_canvas(color),
        }
    }
}