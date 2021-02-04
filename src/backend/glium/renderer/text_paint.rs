use crate::renderer::canvas::*;
use super::renderer::*;
use glium::{
    draw_parameters::*,
    uniform,
    Surface,
};
use rmu::raw::{ Vec2f, Vec4f};

impl TextPaint for GLRenderer {

    fn load_font(&mut self, name: &str, path: &str) {
        self.text_paint.font_set.add(name, path)
    }

    fn set_font(&mut self, name: &str) {
        self.text_paint.active_font = name.to_string()
    }

    fn set_align(&mut self, align: Align) {
        self.text_paint.align = align
    }

    fn set_font_scale(&mut self, scale: Scale) {
        self.text_paint.font_scale = scale
    }

    fn set_font_color(&mut self, color: Vec4f) {
        self.text_paint.font_color = color
    }

    fn set_font_space(&mut self, font_space: f32) {
        self.text_paint.font_space = font_space
    }

    fn set_line_space(&mut self, line_space: f32) {
        self.text_paint.line_space = line_space
    }

    fn set_max_line(&mut self, max_line: u32) {
        self.text_paint.max_line = max_line
    }

    fn set_max_width(&mut self,max_width: f32) {
        self.text_paint.max_width = max_width
    }

    fn set_position(&mut self, position: Vec2f) {
        self.text_paint.postion = position
    }

    fn draw_text(&mut self, text: &str) {

        if let Some(frame) = &mut self.frame {
            use super::font::load_text;
            if let Some(data) = self.text_paint.font_set.font_byte(&self.text_paint.active_font) {
                if let Some(font) = rusttype::Font::try_from_bytes(data) {

                    let (vertex_buffer, index_buffer, tex) = load_text(
                        text, 
                        self.text_paint.font_scale, 
                        self.text_paint.max_width, 
                        self.text_paint.postion,
                        &font, 
                        &self.display
                    );
                    let parameters: DrawParameters = DrawParameters {
                        depth: glium::Depth {
                            test: DepthTest::Overwrite,
                            write: false,
                            ..Default::default()
                        },
                        blend: glium::Blend::alpha_blending(),
                        multisampling: self.antialising_enable,
                        ..Default::default()
                    };

                    frame.draw(
                        &vertex_buffer, 
                        &index_buffer, 
                        &self.screen_data.font, 
                        &uniform! { font_tex: &tex, color: self.text_paint.font_color}, 
                        &parameters
                    ).unwrap();
                }
            }
        }

    }
}