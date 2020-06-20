use crate::base::utils::Size;

#[derive(Debug,Clone)]
pub struct Config {
    pub name      : String,
    pub size      : Size,
    pub v_sync    : bool,
    pub decoration: bool,
    pub resizable : bool,
}

impl Config {
    //
    pub fn create(name: String, width: f32, height: f32) -> Self {
        Self {        
            name,
            size: Size::new(width, height),
            v_sync: true,
            decoration: true,
            resizable: false,
        }
    }

    pub fn aspect_radio(&self) -> f32 {
        self.size.height / self.size.width
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::create("Titanium White".to_string(), 1240.0, 720.0)
    }
}