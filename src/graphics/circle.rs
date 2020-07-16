use rmu::raw::{Vec2f, Vec4f};

#[derive(Clone,Copy)]
pub struct Circle {
    ///anchor is center point
    pub anchor: Vec2f,
    pub radius: f32,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            anchor: [0.9, 0.9],
            radius: 0.1,
        }
    }

    pub fn create(anchor: Vec2f, radius: f32, color: Vec4f) -> Self {
        Self {
            anchor,
            radius,
        }
    }
}

use std::f32::{consts::PI};
// get the point of angle in a position
fn circle(radius: f32 ,position: Vec2f, sampling_times: f32 ) -> Vec<Vec2f> {

    let [x,y] = position;
    let delta = PI * 2.0 / (sampling_times as f32);
    let mut pos: Vec<Vec2f> = Vec::new();
    
    let mut i: f32 = 0.0;
    while i < 2.0 * PI {

        pos.push([x + i.cos() * radius, y + i.sin() * radius]);

        i = i + delta;
    }
    pos
}