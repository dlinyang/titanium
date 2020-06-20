use super::graphics::Graphics;
use super::text::Text;
use crate::base::utils::gen_id;

pub struct Canvas {
    pub id: u64,
    pub layers: Vec<Layer>,
}

pub enum CanvasErr {
    ExitLayer,
}

impl Canvas {
    pub fn new(name: &str) -> Self {
        let id = gen_id(&String::from(name));
        Self {
            id,
            layers: Vec::new(),
        }
    }

    pub fn update(&mut self, layer: Layer) {

        let mut i: usize = 0;

        while i < self.layers.len() {
            if layer.id == self.layers[i].id {
                break;
            } else  {
                i = i + 1;
            }
        }

        if i == self.layers.len() {
            self.layers.push(layer);
        } else {
            self.layers[i] = layer;
        }
    }

    pub fn get_mut_layer(&mut self, name: &String) -> Option<&mut Layer> {
        let id = gen_id(name);

        let mut i: usize = 0;
        
        while i < self.layers.len() {
            if id == self.layers[i].id {
                break;
            } else {
                i = i + 1;
            }
        }

        if i == self.layers.len() {
            None
        } else {
            Some(&mut self.layers[i])
        }
    }

    pub fn remove(&mut self, name: &String)  -> Option<Layer> {
        let id = gen_id(name);

        let mut i: usize = 0;
        
        while i < self.layers.len() {
            if id == self.layers[i].id {
                break;
            } else {
                i = i + 1;
            }
        }

        if i == self.layers.len() {
            None
        } else {
            Some(self.layers.remove(i))
        }
    }
}

#[derive(Clone)]
pub struct Layer{
    pub id: u64,
    pub graphics: Option<Graphics>,
    pub text: Option<Text>,
}


impl Layer {

    pub fn new(name: &str) -> Self {
        let id = gen_id(&String::from(name));

        Self {
            id,
            graphics: None,
            text: None,
        }
    }
    
    pub fn create(name: &str, graphics: Graphics, text: Text) -> Self {
        let id = gen_id(&String::from(name));

        Self {
            id,
            graphics: Some(graphics),
            text: Some(text),
        }
    }

    pub fn graphics(name: &str, graphics: Graphics) -> Self {
        let id = gen_id(&String::from(name));

        Self {
            id,
            graphics: Some(graphics),
            text: None,
        }
    }

    pub fn text(name: &str, text: Text) -> Self {
        let id = gen_id(&String::from(name));

        Self {
            id,
            graphics: None,
            text: Some(text),
        }
    }

    pub fn with_id(id: u64) -> Self {
        Self {
            id,
            graphics:  None,
            text: None,
        }
    }

    pub fn with_graphics(self, graphics: Graphics) -> Self {
        Self {
            id: self.id,
            graphics: Some(graphics),
            text: self.text,
        }
    }

    pub fn with_text(self, text: Text) -> Self {
        Self {
            id: self.id,
            graphics: self.graphics,
            text: Some(text),
        }
    }
}