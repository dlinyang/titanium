use rmu::raw::Vec4f;
use crate::base::Position;
use crate::base::material::Material;

#[derive(Clone,Copy)]
pub enum GraphicsType{Points, Line, LineList, Polygon, PolygonFill}

#[derive(Clone)]
pub struct Graphics {
    pub positions: Vec<Position>,
    pub material: Material,
    pub graphics_type: GraphicsType,
}