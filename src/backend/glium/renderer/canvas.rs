use crate::renderer::{Canvas,RenderProdure};
use super::GLRenderer;

impl Canvas for GLRenderer {
    fn init(&mut self) {
        self.clear();
    }

    fn finish(&mut self) {
        self.swap_buffer();
    }
}