use rmu::raw::{Vec2f,Vec4f};

#[derive(Clone,Copy)]
pub struct Rectangle {
    /// anchor is left upper point
    pub anchor: Vec2f,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    //create a default Rectangle
    pub fn new() -> Self {
        Rectangle {
            anchor: [0.0,0.0],
            w: 0.1,
            h: 0.05,
        }
    }

    pub fn create(anchor: Vec2f,w: f32, h: f32) -> Self {
        Self {
            anchor,
            w,
            h,
        }
    }
}

use super::Graphics2d;

impl Graphics2d for Rectangle {
     #[inline]
    fn positions(&self) -> Vec<Vec2f> {
        let [x,y] = self.anchor;
        let w = self.w;
        let h = self.h;

        let p0 = [x    , y    ];
        let p1 = [x + w, y    ];
        let p2 = [x + w, y + h];
        let p3 = [x    , y + h];
        vec![p0, p1, p2, p3]
    }
}