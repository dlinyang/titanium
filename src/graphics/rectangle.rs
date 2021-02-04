use rmu::raw::Vec2f;

/// return points of  a rectanle
/// the position is lefe upper point
pub fn rectangle(position: Vec2f, w: f32, h: f32) -> Vec<Vec2f> {
        let [x,y] = position;

        let p0 = [x    , y    ];
        let p1 = [x + w, y    ];
        let p2 = [x + w, y + h];
        let p3 = [x    , y + h];
        vec![p0, p1, p2, p3]
}

#[derive(Clone,Copy)]
pub struct Rectangle {
    /// position is left upper point
    pub position: Vec2f,
    /// the width of rectangle
    pub w: f32,
    /// the height of rectangle
    pub h: f32,
}

impl Rectangle {
    pub fn new(position: Vec2f, w: f32, h: f32) -> Self {
        Self {
            position,
            w,
            h,
        }
    }
}

use super::Graphics;

impl Graphics for Rectangle {
     #[inline]
    fn positions(&self) -> Vec<Vec2f> {
        rectangle(self.position, self.w, self.h)
    }
}