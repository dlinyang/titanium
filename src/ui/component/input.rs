use super::super::*;
use rmu::raw::Vec4f;
use crate::base::utils::*;
use crate::renderer::canvas::*;
use crate::event::*;
use crate::event::utils::*;
use crate::graphics::Rectangle;

pub struct Input {
    pub id: u64,
    pub anchor: Anchor,
    pub width: WindowUnit<f32>,
    pub height: WindowUnit<f32>,
    pub area: Area,
    pub input: String,
    pub font: String,
    pub font_size: f32,
    pub font_color: Vec4f,
    pub color: Vec4f,
}

impl Input {
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }
    
    pub fn with_size(mut self, width: WindowUnit<f32>, height: WindowUnit<f32>) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_font(mut self, font: &str) -> Self {
        self.font = font.into();
        self
    }
}

impl WidgetBuilder for Input {
    fn new(name: &str) -> Self {
        Self {
            id: gen_id(&String::from(name)),
            anchor: Default::default(),
            width: Default::default(),
            height: Default::default(),
            area: Default::default(),
            input: String::default(),
            font: String::default(),
            font_size: 1.0,
            font_color: [0.0, 0.0, 0.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    fn build(mut self, ui_state: &mut UIState, canvas: &mut Canvas) -> Self {
        self.area = area(self.anchor, self.width, self.height, ui_state.window_size);
        self.font_size = self.area.height() * ui_state.window_size.height;
        canvas.update(self.layer());
        self
    }
}

impl Widget for Input {
    fn update(&mut self, ui_state: &mut UIState, canvas: &mut Canvas) -> bool {

        match ui_state.event {
            Event::MouseEvent { button, state} => match button {
                MouseButton::Left => if state == ButtonState::Press {
                    if self.area.in_area(ui_state.cursor[0], ui_state.cursor[1]) {
                        ui_state.active_widget = Some(self.id);
                    } else {
                        ui_state.none_active(self.id);
                    }
                },
                _ => (),
            },
            _ => (),
        }
        
        if ui_state.is_active(self.id) {
            if let Event::Character(c) = ui_state.event {
                if c == '\u{8}' || c == '\u{7f}'{
                    false
                } else {
                    self.input.push(c);
                    canvas.update(self.layer());
                    true
                }
            } else if let Event::KeyEvent{ key, state} = ui_state.event {
                if state == ButtonState::Press && (key == Key::Delete || key == Key::Back) {
                    self.input.pop();
                    canvas.update(self.layer());
                    true
                } else{
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn layer(&self) -> Layer {
        let anchor = self.area.top_left_point;
        let width = self.area.width();
        let height = self.area.height();

        let graphics = Rectangle::create(anchor, width, height, self.color, graphics::GraphicsType::Polygon);
        let text = text::Text::create(self.input.clone(), self.font.clone(), anchor, Size::uniform(self.font_size), width, self.font_color);

        Layer::with_id(self.id)
              .with_graphics(graphics.into())
              .with_text(text)
    }

    fn id(&self) -> u64 {
        self.id
    }
}