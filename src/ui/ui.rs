use crate::base::utils::*;
use crate::event::*;
use rmu::raw::Vec2f;

pub struct UIState{
    pub window_size: Size,
    pub hot_widget: Option<u64>,
    pub active_widget: Option<u64>,
    pub event: Event,
    pub cursor:  Vec2f,
}


impl UIState {
    pub fn new(window_size: Size) -> Self {
        Self {
            window_size,
            hot_widget: None,
            active_widget: None,
            event: Event::Other,
            cursor: [0.0, 0.0],
        }
    }

    pub fn update(&mut self, event: Event) {
        self.event = event;

        if let Event::CursorEvent{x,y} = event {
            self.cursor = [x / self.window_size.width, y / self.window_size.height];
        }
    }

    pub fn is_hot(&self, id: u64) -> bool {
        if let Some(hot_id) = self.hot_widget {
            if hot_id == id {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn none_hot(&mut self, id: u64) {
        if let Some(hot_id) = self.hot_widget {
            if id == hot_id {
                self.hot_widget = None;
            }
        }
    }

    pub fn is_active(&self, id: u64) -> bool {
        if let Some(active_id) = self.active_widget {
            if active_id == id {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn none_active(&mut self, id: u64)  {
        if let Some(active_id) = self.active_widget {
            if id == active_id {
                self.active_widget = None;
            }
        }
    }
}

