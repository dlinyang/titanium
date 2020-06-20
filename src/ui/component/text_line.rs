use super::super::*;
use rmu::raw::Vec4f;
use crate::base::utils::*;
use crate::renderer::canvas::*;
use crate::event::*;
use crate::event::utils::*;

pub struct TextLine {
    pub id: u64,
    pub anchor: Anchor,
    pub width: f32,
    pub context: String,
    pub font: String,
    pub font_color: Vec4f,
    pub font_size: f32,
}

impl TextLine {
    pub fn with_font(mut self, font: &str) -> Self {
        self.font = font.into();
        self
    }

    pub fn with_font_color(mut self, font_color: Vec4f) -> Self {
        self.font_color = font_color;
        self
    }
}

impl WidgetBuilder for TextLine {
    fn new(name: &str, anchor: Anchor) -> Self {
        Self {
            id: gen_id(&String::from(name)),
            anchor,
            width: Default::default(),
            font: Default::default(),
            font_color: [0.0, 0.0, 0.0, 1.0],
            font_size: 1.0,
        }
    }

    fn build(mut self, ui_state: &mut UIState, canvas: &mut Canvas)  -> Self {
        self.area = area(self.anchor, self.width, self.height, ui_state.window_size);
        self.font_size = self.area.height() * ui_state.window_size.height;
        canvas.update(self.layer());
        self
    }
}

use crate::graphics::Rectangle;

impl Widget for Button {
    fn update(&mut self, ui_state: &mut UIState, canvas: &mut Canvas) -> bool {

        if self.area.in_area(ui_state.cursor[0], ui_state.cursor[1]) {
            ui_state.hot_widget = Some(self.id);

            if let Event::MouseEvent{state, button} = ui_state.event {
                if button == MouseButton::Left {
                    self.click.click(state);
                }
            }

        } else {
            ui_state.none_hot(self.id);
        }

        if ui_state.is_hot(self.id) {
            if self.is_hover {
                false
            } else {
                self.is_hover = true;
                canvas.update(self.layer());
                true
            }
        } else {
            if self.is_hover {
                self.is_hover = false;
                canvas.update(self.layer());
                true
            } else {
                false
            }
        }
    }

    fn layer(&self) -> Layer {

        let [r,g,b,a] = self.color;
        let color = if self.is_hover { 
                [r / 2.0, g / 2.0, b / 2.0, a]
            } else {
                [r, g, b, a]
            };
            
        let width = self.area.width();
        let height = self.area.height();
        let text_width = height * 0.5 * self.label.len() as f32;

        let text_anchor = [self.area.top_left_point[0] + ( width -  text_width )  * 0.5, self.area.top_left_point[1]];

        Layer::with_id(self.id)
              .with_graphics(Rectangle::create(self.area.top_left_point, width, height, color, graphics::GraphicsType::Polygon).into())
              .with_text(
                  text::Text::create(
                      self.label.clone(), 
                      self.font.clone(), text_anchor, 
                      Size::new(self.font_size * 0.5 , self.font_size), 
                      text_width, 
                      self.font_color
                    )
                )
    }

    fn id(&self) -> u64 {
        self.id
    }
}