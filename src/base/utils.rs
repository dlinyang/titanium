use rmu::raw::Vec2f;

#[derive(Copy,Clone)]
pub struct Area {
    pub top_left_point: Vec2f,
    pub bottom_right_point: Vec2f,
}

impl Area {
    pub fn height(&self) -> f32 {
        self.bottom_right_point[1] - self.top_left_point[1]
    }

    pub fn width(&self) -> f32 {
        self.bottom_right_point[0] - self.top_left_point[0]
    }

    pub fn in_area(&self, x: f32, y: f32)  -> bool {
        if (self.top_left_point[0] < x) 
            & (x < self.bottom_right_point[0]) 
            & (self.top_left_point[1] < y)
            & (y < self.bottom_right_point[1]){
            true
        } else {
            false
        }
    }
}

impl Default for Area {
    fn default() -> Self{
        Self{
            top_left_point: [0.0, 0.0],
            bottom_right_point: [0.0, 0.0],
        }
    }
}

/// a data with height and width 
#[derive(Debug,Clone,Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }

    pub fn uniform(v: f32) -> Self {
        Self {
            width: v,
            height: v,
        }
    }
}

impl Default for Size {
    fn default() -> Size {
        Size {
            width: 0.0,
            height: 0.0,
        }
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash, Hasher};

pub fn gen_id(name: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish()
}