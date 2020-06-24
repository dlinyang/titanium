use super::super::*;
use rmu::raw::Vec4f;
use crate::base::utils::*;
use crate::renderer::canvas::*;
use crate::event::*;
use crate::event::utils::*;

pub struct Button {
    pub id: u64,
    pub anchor: Anchor,
    pub width: WindowUnit<f32>,
    pub height: WindowUnit<f32>,
    pub area: Area,
    pub label: String,
    pub color: Vec4f,
    pub font: String,
    pub font_color: Vec4f,
    pub font_size: f32,
    pub is_hover: bool,
    pub click: Click,
}

impl Button {
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn with_size(mut self, width: WindowUnit<f32>, height: WindowUnit<f32>) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_font(mut self, font: &str) -> Self {
        self.font = font.into();
        self
    }

    pub fn with_font_color(mut self, font_color: Vec4f) -> Self {
        self.font_color = font_color;
        self
    }

    pub fn with_color(mut self, color: Vec4f) -> Self {
        self.color = color;
        self
    }


    pub fn is_click(&mut self) -> bool {
        self.click.is_click()
    }
}

impl WidgetBuilder for Button {
    fn new(name: &str) -> Self {
        Self {
            id: gen_id(&String::from(name)),
            anchor: Anchor::default(),
            width: Default::default(),
            height: Default::default(),
            area: Default::default(),
            label: Default::default(),
            color: [1.0, 1.0, 1.0, 1.0],
            font: Default::default(),
            font_color: [0.0, 0.0, 0.0, 1.0],
            font_size: 1.0,
            is_hover: false,
            click: Click::Release,
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
              .with_graphics(Rectangle::create(self.area.top_left_point, width, height, color, graphics::GraphicsType::PolygonFill).into())
              .with_text(
                  text::Text::create(
                      self.label.clone(), 
                      self.font.clone(), text_anchor, 
                      Size::uniform(self.font_size), 
                      text_width, 
                      self.font_color
                    )
                )
    }

    fn id(&self) -> u64 {
        self.id
    }
}