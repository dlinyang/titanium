use crate::renderer::*;
use crate::base::{Position, ImagePosition};
use super::GLRenderer;
use rmu::raw::{Vec2f, Vec4f};
use glium::{VertexBuffer, index::*, Surface, uniform, draw_parameters::*};

impl Renderer2D for GLRenderer {
    fn load_font(&mut self, name: &str, path: &str) {
        self.font_set.add(name, path);
    }
    //
    fn draw_points(&mut self, positions: Vec<Vec2f>, color: Vec4f, size: f32) {

        if let Some(frame) = &mut self.frame {

            let positions = to_ndc(positions);
            let vertex_buffer = VertexBuffer::new(&self.display, &positions).unwrap();
            let indices = NoIndices(PrimitiveType::Points);

            let parameters: DrawParameters = DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                multisampling: self.antialising_enable,
                point_size: Some(size),
                ..Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &self.screen_data.color, &uniform! { color : color}, &parameters).unwrap();
        }

    }
    
    //
    fn draw_line(&mut self, positions: Vec<Vec2f>, color: Vec4f, size: f32) {
        if let Some(frame) = &mut self.frame {

            let positions = to_ndc(positions);
            let vertex_buffer = VertexBuffer::new(&self.display, &positions).unwrap();
            let indices = NoIndices(PrimitiveType::LineStrip);

            let parameters: DrawParameters = DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                multisampling: self.antialising_enable,
                line_width: Some(size),
                ..Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &self.screen_data.color, &uniform! { color : color}, &parameters).unwrap();
        }
    }

    fn draw_lines(&mut self, positions: Vec<Vec2f>, color: Vec4f, size: f32) {

        if let Some(frame) = &mut self.frame {

            let positions = to_ndc(positions);
            let vertex_buffer = VertexBuffer::new(&self.display, &positions).unwrap();
            let indices = NoIndices(PrimitiveType::LinesList);

            let parameters: DrawParameters = DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                multisampling: self.antialising_enable,
                line_width: Some(size),
                ..Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &self.screen_data.color, &uniform! { color : color}, &parameters).unwrap();
        }
    }

    fn draw_polygon(&mut self, positions: Vec<Vec2f>, color: Vec4f, size: f32) {
        if let Some(frame) = &mut self.frame {

            let positions = to_ndc(positions);
            let vertex_buffer = VertexBuffer::new(&self.display, &positions).unwrap();
            let indices = NoIndices(PrimitiveType::LineLoop);

            let parameters: DrawParameters = DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                multisampling: self.antialising_enable,
                line_width: Some(size),
                ..Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &self.screen_data.color, &uniform! { color : color}, &parameters).unwrap();
        }
    }

    fn draw_polygon_fill(&mut self, positions: Vec<Vec2f>, color: Vec4f) {
        if let Some(frame) = &mut self.frame {

            let positions = to_ndc(positions);
            let vertex_buffer = VertexBuffer::new(&self.display, &positions).unwrap();
            let indices = NoIndices(PrimitiveType::TriangleFan);

            let parameters: DrawParameters = DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                multisampling: self.antialising_enable,
                ..Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &self.screen_data.color, &uniform! { color : color}, &parameters).unwrap();
        }
    }

    fn set_pixel(&mut self, position: Vec2f, color: Vec4f) {
        if let Some(frame) = &mut self.frame {

            let vertex_buffer = VertexBuffer::new(
                &self.display, 
                &[Position::new(ndc(position))]
            ).unwrap();

            let indices = NoIndices(PrimitiveType::Points);

            let parameters: DrawParameters = DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                multisampling: self.antialising_enable,
                ..Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &self.screen_data.image, &uniform! {color: color}, &parameters).unwrap();
        }
    }

    fn draw_image(&mut self, data: ImageData, positions: [Vec2f;4], uv: [Vec2f;4]) {
        if let Some(frame) = &mut self.frame {

            use glium::texture::*;
            use std::borrow::Cow;

            let image = Texture2d::with_format(
                &self.display,
                RawImage2d {
                    data: Cow::Owned(data.data),
                    width: data.dimensions.0,
                    height: data.dimensions.1,
                    format: data.image_type.into(),
                },
                data.image_type.into(),
                MipmapsOption::NoMipmap,
            ).unwrap();

            let vertex_buffer = VertexBuffer::new(
                &self.display, 
                &[
                    ImagePosition::new(ndc(positions[0]), uv[0]),
                    ImagePosition::new(ndc(positions[1]), uv[1]),
                    ImagePosition::new(ndc(positions[2]), uv[2]), 
                    ImagePosition::new(ndc(positions[3]), uv[3]),
                ]
            ).unwrap();

            let indices = NoIndices(PrimitiveType::TriangleFan);

            let parameters: DrawParameters = DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::Overwrite,
                    write: false,
                    ..Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                multisampling: self.antialising_enable,
                ..Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &self.screen_data.image, &uniform! { tex: &image }, &parameters).unwrap();
        }
    }

    fn draw_text(&mut self, text: &Text, color: Vec4f) {

        if let Some(frame) = &mut self.frame {
            use super::font::load_text;
            if let Some(data) = self.font_set.font_byte(&text.font) {
                if let Some(font) = rusttype::Font::try_from_bytes(data) {

                    let (vertex_buffer, index_buffer, tex) = load_text(text, &font, &self.display);
                    let parameters: DrawParameters = DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::Overwrite,
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
                        &uniform! { font_tex: &tex, color: color}, 
                        &parameters
                    ).unwrap();
                }
            }
        }

    }

    fn load_sprite(&mut self, data: ImageData, name: &String) {
        self.update_texture(data, name);
    }

    fn remove_sprite(&mut self, name: &String) {
        self.remove_texture(name);
    }

    fn draw_sprite(&mut self, name: &String, positions: [Vec2f;4], uv: [Vec2f;4]) {
        if let Some(sprite) = self.data_buffer.texture_buffer.get(name) {
            if let Some(frame) = &mut self.frame {
                let vertex_buffer = VertexBuffer::new(
                    &self.display, 
                    &[
                        ImagePosition::new(ndc(positions[0]), uv[0]),
                        ImagePosition::new(ndc(positions[1]), uv[1]),
                        ImagePosition::new(ndc(positions[2]), uv[2]), 
                        ImagePosition::new(ndc(positions[3]), uv[3]),
                    ]
                ).unwrap();

                let indices = NoIndices(PrimitiveType::TriangleFan);

                let parameters: DrawParameters = DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::Overwrite,
                        write: false,
                        ..Default::default()
                    },
                    blend: glium::Blend::alpha_blending(),
                    multisampling: self.antialising_enable,
                    ..Default::default()
                };

                frame.draw(&vertex_buffer, &indices, &self.screen_data.image, &uniform! { tex: sprite }, &parameters).unwrap();
            }
        }
    }
}

fn to_ndc(postions: Vec<Vec2f>) -> Vec<Position> {
    postions.iter().map( |[x,y]| {Position::new([2.0 * x - 1.0, -2.0 * y + 1.0])}).collect()
}

fn ndc(position: Vec2f) -> Vec2f {
    [2.0 * position[0] - 1.0, -2.0 * position[1] + 1.0]
}