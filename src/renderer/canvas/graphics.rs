use rmu::raw::Vec4f;
use crate::base::Position;
use crate::base::material::Material;

#[derive(Clone,Copy)]
pub enum GraphicsType{Points, Line, Loop, Polygon}

#[derive(Clone)]
pub struct Graphics {
    pub positions: Vec<Position>,
    pub material: Material,
    pub graphics_type: GraphicsType,
}

impl Graphics{

    #[inline]
    pub fn get_indices(&self) -> Vec<u32> {
        match &self.graphics_type {
            GraphicsType::Points => points(&self.positions),
            GraphicsType::Line => line(&self.positions),
            GraphicsType::Loop => line_loop(&self.positions),
            GraphicsType::Polygon => polygon(&self.positions),
        }
    }
}

fn points(positions: &Vec<Position>) -> Vec<u32> {
    let mut result = Vec::new();

    for x in 0..(positions.len()-1) {
        result.push(x as u32);
    }

    result
}

fn line(positions: &Vec<Position>) -> Vec<u32> {
    points(positions)
}

fn line_loop(positions: &Vec<Position>) -> Vec<u32> {
    points(positions)
}

fn polygon(positions: &Vec<Position>) -> Vec<u32> {
    let mut result = Vec::new();

    let mut x: u32 = 1;

    while x < positions.len() as u32 - 1 {
        result.push(0);
        result.push(x);
        result.push(x+1);
        x = x + 1;
    }
    result
}

