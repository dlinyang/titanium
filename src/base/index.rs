#[derive(Clone,PartialEq,PartialOrd)]
pub enum Index {
    TriangleFace(Vec<u32>),
    EdgeLists(Vec<u32>),
    Points(Vec<u32>),
}