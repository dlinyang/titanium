use crate::base::utils::Size;

use rmu::raw::{Vec2f,Vec4f};
//

#[derive(Clone)]
pub struct Text {
    pub context: String,
    pub font: String,
    pub position: Vec2f,
    pub size: Size,
    pub width: f32,
}

pub enum Align {
    Left,
    Center,
    Right,
}

impl Text {
    pub fn new(context: String) -> Self {
        Self {
            context,
            font: String::default(),
            position: [0.0,0.0],
            size: Size {width: 8.0, height:8.0},
            width: 1.0,
        }
    }

    pub fn create(context: String, font: String ,position:Vec2f, size: Size, width: f32) -> Self{
        Self {
            context,
            font,
            position,
            size,
            width,
        }
    }
}