use super::{Widget, UIState};
use crate::renderer::Canvas;

pub trait WidgetSet<R> where R: Canvas {
    fn new() -> Self;
    fn update(&mut self, ui_state: &mut UIState) -> bool;
    fn render(&self, renderer: &mut R);
}

pub struct SimpleWidgetSet<R> where R: Canvas{
    pub widgets: Vec<Box<dyn Widget<R>>>,
    pub callback: Option<Box<dyn FnMut(&mut UIState)>>,
}

impl<R> WidgetSet<R> for SimpleWidgetSet<R>  where R: Canvas {
    fn new() -> Self {
        Self {
            widgets: Vec::new(),
            callback: None,
        }
    }

    fn update(&mut self, ui_state: &mut UIState) -> bool {
        let mut flag = false;

        for widget in &mut self.widgets {
            if widget.update(ui_state) {
                if let Some(callback) = &mut self.callback {
                    callback(ui_state);
                    flag = true;
                }
            }
        }

        flag
    }

    fn render(&self, renderer: &mut  R) {
        for widget in &self.widgets {
            widget.render(renderer);
        }
    }
}