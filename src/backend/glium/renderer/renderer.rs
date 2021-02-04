use crate::application::*;
use crate::renderer::*;
use crate::base::font::FontSet;

use super::buffer::*;
use super::pipeline::*;
use super::screen_data::*;
use glium::Display;
use glium::Frame;

use std::rc::Rc;

pub struct GLRenderer {
    pub display           : Display,
    pub frame             : Option<Frame>,
    pub data_buffer       : DataBuffer,
    pub shader_buffer     : Rc<GLShaderBuffer>,
    pub screen_data       : ScreenData,
    pub antialising_enable: bool,
    pub config            : Config,
    pub hdr_enable        : bool,
    pub gamma             : f32,
    pub graphics_paint    : GLGraphicsPaint,
    pub text_paint        : GLTextPaint,
}

impl GLRenderer {
    pub fn new(config: Config, display: Display) -> Self {
        let mut shader_buffer = GLShaderBuffer::new();
        shader_buffer.load_bulidin(&display);

        Self {
            frame             : None,
            data_buffer       : DataBuffer::new(&display),
            shader_buffer     : Rc::new(shader_buffer),
            screen_data       : ScreenData::new(&display),
            antialising_enable: true,
            display,
            config,
            hdr_enable        : true,
            gamma             : 2.2,
            graphics_paint    : GLGraphicsPaint::new(),
            text_paint        : GLTextPaint::new(),
        }
    }
}

use rmu::raw::{ Vec2f, Vec4f};

impl Renderer for GLRenderer {

    fn background_color(&mut self, color: Vec4f) {
        self.data_buffer.bg_color = color;
    }

    fn set_antialiasing(&mut self, enable: bool) {
        self.antialising_enable = enable;
    }

    fn set_hdr(&mut self, enable: bool) {
        self.hdr_enable = enable;
    }

    fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }
}

use crate::base::color::BLACK;

pub struct GLGraphicsPaint {
    pub size: f32,
    pub line_width: f32,
    pub color: Vec4f,
}

impl GLGraphicsPaint {
    pub fn new() -> Self {
        Self {
            size: 1f32,
            line_width: 1f32,
            color: BLACK,
        }
    }
}

pub struct GLTextPaint {
    pub font_set: FontSet,
    pub active_font: String,
    pub font_scale: Scale,
    pub font_color: Vec4f,
    pub font_space: f32,
    pub line_space: f32,
    pub align: Align,
    pub max_line: u32,
    pub max_width: f32,
    pub postion: Vec2f,
}

impl GLTextPaint {
    pub fn new() -> Self {
        Self {
            font_set: FontSet::new(),
            active_font: String::default(),
            font_scale: Scale::uniform(14f32),
            font_color: BLACK,
            font_space: 0f32,
            line_space: 0f32,
            align: Align::Left,
            max_line: 1u32,
            max_width: 100f32,
            postion: [0f32;2]
        }
    }
}