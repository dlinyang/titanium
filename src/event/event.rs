use super::Key;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum Event {
    MouseEvent {
        state: ButtonState,
        button: MouseButton,
    },
    CursorEvent {x: f32, y: f32},
    KeyEvent {
        state: ButtonState,
        key: Key,
    },
    Character(char),
    Exit,
    Other,
}

pub trait EventSystem where {
    fn event(&mut self) -> Event;
    fn device_event(&mut self) -> Event;
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum ButtonState {
    Press,
    Release,
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other,
}