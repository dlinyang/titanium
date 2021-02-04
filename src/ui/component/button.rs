use super::super::*;
use rmu::raw::Vec4f;
use crate::base::utils::*;
use crate::event::*;
use crate::event::utils::*;
use crate::graphics::round_rectangle::*;
use crate::graphics::Graphics2d;

pub struct Button {
    pub id: u64,
    pub anchor: Anchor,
    pub width: WindowUnit<f32>,
    pub height: WindowUnit<f32>,
    pub round_angle: RoundAngle,
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

    /// Round angle radius unit : pixel
    pub fn with_round_angle(mut self, round_angle: RoundAngle) -> Self {
        self.round_angle = round_angle;
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
            round_angle: Default::default(),
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

    fn build(mut self, ui_state: &mut UIState)  -> Self {
        self.area = area(self.anchor, self.width, self.height, ui_state.window_size);
        self.round_angle.div(ui_state.window_size.height);
        self.font_size = self.area.height() * ui_state.window_size.height;
        self
    }
}

impl WidgetAction for Button  {
    fn update(&mut self, ui_state: &mut UIState) -> bool {

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
                true
            }
        } else {
            if self.is_hover {
                self.is_hover = false;
                true
            } else {
                false
            }
        }
    }

    fn id(&self) -> u64 {
        self.id
    }
}

use crate::renderer::{Canvas, Text};

impl<R> WidgetRender<R> for Button where R: Canvas{
    fn render(&self, renderer: &mut R) { 
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

        let graphics = RoundRectangle::create(self.area.top_left_point, width, height, self.round_angle);

        let text = Text::create( self.label.clone(), self.font.clone(), text_anchor, Size::uniform(self.font_size), text_width);
        
        renderer.set_color(color);
        renderer.draw_polygon_fill(graphics.positions());
        renderer.set_font_color(font_color);
        renderer.draw_text(&text);
    }
}

impl<R> Widget<R> for Button where R: Canvas {

}