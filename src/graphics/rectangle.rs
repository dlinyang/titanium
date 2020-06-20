use rmu::raw::{Vec2f,Vec4f};
use crate::base::Position;
use crate::base::material::color_canvas;
use crate::renderer::canvas::graphics::{Graphics,GraphicsType};

#[derive(Clone,Copy)]
pub struct Rectangle {
    /// anchor is left upper point
    pub anchor: Vec2f,
    pub w: f32,
    pub h: f32,
    pub color: Vec4f,
    pub graphics_type: GraphicsType,
}

impl Rectangle {
    //create a default Rectangle
    pub fn new() -> Self {
        Rectangle {
            anchor: [0.0,0.0],
            w: 0.1,
            h: 0.05,
            color: [0.1, 0.1, 0.5, 1.0],
            graphics_type: GraphicsType::Polygon,
        }
    }

    pub fn create(anchor: Vec2f,w: f32, h: f32, color: Vec4f, graphics_type: GraphicsType) -> Self {
        Self {
            anchor,
            w,
            h,
            color,
            graphics_type,
        }
    }
}

impl Into<Graphics> for Rectangle {
    fn into(self) -> Graphics {
        Graphics {
            positions: rectangle(self.anchor,self.w,self.h),
            material: color_canvas(self.color),
            graphics_type: self.graphics_type,
        }
    }
}

#[inline]
fn rectangle(position: [f32;2], w: f32, h: f32) -> Vec<Position> {
    let [x,y] = position;
    let p0 = Position::from(position);
    let p1 = Position::from([x + w, y]);
    let p2 = Position::from([x + w, y + h]);
    let p3 = Position::from([x,y + h]);
    vec![p0, p1, p2, p3]
}