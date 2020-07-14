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
            graphics_type: GraphicsType::PolygonFill,
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

    #[inline]
    fn position(&self) -> Vec<Position> {
        let [x,y] = self.anchor;
        let w = self.w;
        let h = self.h;

        let p0 = Position::from([x,y]);
        let p1 = Position::from([x + w, y]);
        let p2 = Position::from([x + w, y + h]);
        let p3 = Position::from([x,y + h]);
        vec![p0, p1, p2, p3]
    }
}

impl From<Rectangle> for Graphics {
    fn from(rectangle: Rectangle) -> Self {
        Self {
            positions: rectangle.position(),
            material: color_canvas(rectangle.color),
            graphics_type: rectangle.graphics_type,
        }
    }
}