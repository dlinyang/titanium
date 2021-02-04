use crate::{
    base::Position,
    renderer::{ GraphicsPaint, Image, RendererManager},
};
use super::renderer::GLRenderer;
use glium::index::{ NoIndices, PrimitiveType};
use glium::vertex::VertexBuffer;
use glium::draw_parameters::DrawParameters;
use glium::uniform;
use glium::Surface;
use rmu::raw::{ Vec2f, Vec4f};

impl GraphicsPaint for GLRenderer {

    fn set_size(&mut self, size: f32) {
        self.graphics_paint.size = size
    }

    fn set_color(&mut self, color: Vec4f) {
        self.graphics_paint.color = color
    }

    fn set_line_width(&mut self, line_width: f32) {
        self.graphics_paint.line_width = line_width
    }

    fn draw_points(&mut self, positions: Vec<Vec2f>) {

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
                point_size: Some(self.graphics_paint.size),
                ..Default::default()
            };

            frame.draw(&vertex_buffer, &indices, &self.screen_data.color, &uniform! { color : self.graphics_paint.color}, &parameters).unwrap();
        }

    }
    
    //
    fn draw_line(&mut self, positions: Vec<Vec2f>) {
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
                line_width: Some(self.graphics_paint.line_width),
                ..Default::default()
            };

            frame.draw(
                &vertex_buffer, 
                &indices, 
                &self.screen_data.color, 
                &uniform!{ color : self.graphics_paint.color }, 
                &parameters
            ).unwrap()
        }
    }

    fn draw_polygon(&mut self, positions: Vec<Vec2f>) {
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
                line_width: Some(self.graphics_paint.line_width),
                ..Default::default()
            };

            frame.draw(
                &vertex_buffer, 
                &indices, 
                &self.screen_data.color, 
                &uniform!{ color : self.graphics_paint.color }, 
                &parameters
            ).unwrap()
        }
    }

    fn draw_polygon_fill(&mut self, positions: Vec<Vec2f>) {
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

            frame.draw(&vertex_buffer, &indices, &self.screen_data.color, &uniform! { color : self.graphics_paint.color}, &parameters).unwrap();
        }
    }

    fn load_sprite(&mut self, name: &str, image: &Image) {
        self.update_texture(name, image);
    }

    fn remove_sprite(&mut self, name: &str) {
        self.remove_texture(name);
    }

    fn draw_image(&mut self, position_uvs: Vec<(Vec2f,Vec2f)>, sprite_name: &str) {
        if let Some(sprite) = self.data_buffer.texture_buffer.get(sprite_name) {
            let vertex: Vec<Position> = position_uvs
                .iter()
                .map( |(position,uv)| {Position::new(*position, *uv)})
                .collect();

            if let Some(frame) = &mut self.frame {
                let vertex_buffer = VertexBuffer::new(&self.display, &vertex).unwrap();

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

                frame.draw(&vertex_buffer, &indices, &self.screen_data.image, &uniform!{ tex: sprite }, &parameters).unwrap();
            }
        }
    }
}

fn to_ndc(postions: Vec<Vec2f>) -> Vec<Position> {
    postions.iter().map( |[x,y]| {Position::from([2.0 * x - 1.0, -2.0 * y + 1.0])}).collect()
}