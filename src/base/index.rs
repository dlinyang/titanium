#[derive(Clone,PartialEq,PartialOrd)]
pub enum Indices {
    TriangleFace(Vec<u32>),
    EdgeLists(Vec<u32>),
    Points(Vec<u32>),
}