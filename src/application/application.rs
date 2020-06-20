///The Application struct include the basic function to create a graphic program
/// 
/// #Example
/// 
/// ```rust
/// use titanium::*
/// 
/// let config = Config::new();
/// let application = Application::mew(config);
/// 
/// ```

use crate::renderer::Renderer;
use crate::event::EventSystem;
use super::config::Config;

#[derive(Copy,Clone,PartialEq,PartialOrd)]
pub enum LoopControl {
    Continue,
    Exit,
}

pub trait Application<R,E> where R: Renderer , E :EventSystem {
    fn new(config: Config) -> Self;
    fn rendering_loop<F: FnMut(&mut R,&mut E,&mut LoopControl)>(&mut self, f: F);
    fn rendering<F: FnMut(&mut R, &mut E)>  (&mut self, f:F);
}