/// A triat include the basic function to create a graphic program
/// 
/// #Example
/// 
/// ```ignore
/// use titanium::prelude::*;
/// use titanium::backend::glium::*;
/// 
/// let config = Config::default();
/// let application = Application::new(config);
/// 
/// ```

use crate::renderer::{Renderer,Canvas};
use crate::event::EventSystem;
use super::config::Config;

pub trait Application<R,E> where R: Renderer + Canvas , E :EventSystem {
    fn new(config: Config) -> Self;
    fn rendering_loop<F: FnMut(&mut R,&mut E,&mut LoopControl)>(&mut self, f: F);
    fn rendering<F: FnMut(&mut R, &mut E)>  (&mut self, f:F);
}

#[derive(Copy,Clone,PartialEq,PartialOrd)]
pub enum LoopControl {
    Continue,
    Exit,
}