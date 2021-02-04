use rmu::raw::Vec2f;

use std::f32::{consts::PI};

/// Get the points of circle,
///  o is center of circle, r is radius, n is sampling time
pub fn circle(o: Vec2f, r: f32, n: u32 ) -> Vec<Vec2f> {

    let [x,y] = o;
    let delta = PI * 2.0 / (n as f32);
    let mut pos: Vec<Vec2f> = Vec::new();
    
    let mut i: f32 = 0.0;
    while i < 2.0 * PI {

        pos.push([x + i.cos() * r, y + i.sin() * r]);

        i = i + delta;
    }
    pos
}

#[derive(Clone,Copy)]
pub struct Circle {
    ///postion is center point
    pub o: Vec2f,
    pub r: f32,
}

impl Circle {
    pub fn new(o: Vec2f, r: f32) -> Self {
        Self {
            o,
            r,
        }
    }
}

use super::Graphics;

impl Graphics for Circle {
    fn positions(&self) -> Vec<Vec2f> {
        circle(self.o, self.r, (self.r * 100f32) as u32)
    }
}