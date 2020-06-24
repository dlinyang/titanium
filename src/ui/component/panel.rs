use super::super::*;
use rmu::raw::Vec4f;
use crate::base::utils::*;
use crate::renderer::canvas::*;
use crate::event::*;
use crate::event::utils::*;
use crate::graphics::Rectangle;

pub struct Panel {
    pub id: u64,
    pub anchor: Anchor,
    pub width: WindowUnit<f32>,
    pub height: WindowUnit<f32>,
    pub area: Area,
    pub color: Vec4f,
    pub drag: Drag,
}

impl Panel {
    pub fn with_anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }
    
    pub fn with_size(mut self, width: WindowUnit<f32>, height: WindowUnit<f32>) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_color(mut self, color: Vec4f) -> Self {
        self.color = color;
        self
    }
}

impl WidgetBuilder for Panel {
    fn new(name: &str) -> Self {
       Self {
            id: gen_id(&String::from(name)),
            anchor: Anchor::default(),
            width: Default::default(),
            height: Default::default(),
            area: Default::default(),
            color: [1.0, 1.0, 1.0, 1.0],
            drag: Drag::new(MouseButton::Left),
        }
    }

    fn build(mut self, ui_state: &mut UIState, canvas: &mut Canvas) -> Self {
        self.area = area(self.anchor, self.width, self.height, ui_state.window_size);
        canvas.update(self.layer());
        self
    }
}

impl Widget for Panel {
    fn update(&mut self, ui_state: &mut UIState, canvas: &mut Canvas) -> bool {

        self.drag.match_event(ui_state.event);

        if self.area.in_area(ui_state.cursor[0], ui_state.cursor[1]) {
            match ui_state.event {
                Event::MouseEvent { button, state} => match button {
                    MouseButton::Left => match state {
                        ButtonState::Press => ui_state.active_widget = Some(self.id),
                        ButtonState::Release => ui_state.none_active(self.id),
                    },
                    _ => (),
                }
                _ => (),
            }
        }

        if ui_state.is_active(self.id) {
            if self.drag.is_move() {
                let m = self.drag.get_move();

                match self.anchor.value() {
                    WindowUnit::Pixel([x,y]) => self.anchor.set(WindowUnit::Pixel([x + m[0], y + m[1]])),
                    WindowUnit::Point([x,y]) => self.anchor.set(WindowUnit::Pixel([x + m[0] / ui_state.window_size.width, y + m[1] / ui_state.window_size.height])),
                }

                self.area = area(self.anchor, self.width, self.height, ui_state.window_size);

                canvas.update(self.layer());
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn layer(&self) ->  Layer {
        let anchor = self.area.top_left_point;
        let width = self.area.width();
        let height = self.area.height();

        Layer::with_id( self.id)
              .with_graphics(Rectangle::create(anchor, width, height, self.color, graphics::GraphicsType::PolygonFill).into())
    }

    fn id(&self) -> u64 {
        self.id
    }
}