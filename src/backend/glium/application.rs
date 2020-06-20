use crate::application::*;

use glium::glutin::{
    event_loop::EventLoop,
    window::WindowBuilder,
    ContextBuilder,
    dpi::LogicalSize};

use glium::Display;

use super::event::GLEventSystem;
use super::renderer::GLRenderer;

pub struct GLApplication {
    pub config: Config,
    pub event_system: GLEventSystem,
    pub renderer: GLRenderer,
}

impl Application<GLRenderer,GLEventSystem> for GLApplication {
    //
    fn new(config: Config) -> GLApplication {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(config.name.clone())
            .with_inner_size(LogicalSize::new(config.size.width as f64, config.size.height as f64))
            .with_decorations(config.decoration)
            .with_resizable(config.resizable);
        let context_builder = ContextBuilder::new().with_depth_buffer(24).with_vsync(config.v_sync);
        let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

        GLApplication {
            event_system: GLEventSystem::new(event_loop),
            renderer: GLRenderer::new(config.clone(), display),
            config
        }
    }

    fn rendering_loop<F: FnMut(&mut GLRenderer,&mut GLEventSystem,&mut LoopControl)>(&mut self,mut f: F) {
        let mut control = LoopControl::Continue;
        while control != LoopControl::Exit {
            f(&mut self.renderer,&mut self.event_system,&mut control);
        }
    }

    fn rendering<F: FnMut(&mut GLRenderer, &mut GLEventSystem)>( &mut self, mut f: F) {
        f(&mut self.renderer, &mut self.event_system);
    }
}