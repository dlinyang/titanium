pub struct Image {
    pub data: Vec<u8>,
    pub dimensions: (u32,u32),
    pub image_type: ImageType,
}

#[derive(Copy,Clone)]
pub enum ImageType {
    U8,
    U8U8U8,
    U8U8U8U8,
}