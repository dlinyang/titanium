use std::collections::HashMap;
use std::fs::read;

pub struct FontSet {
    pub data: HashMap<String,Vec<u8>>
}

impl FontSet {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self,name: &str, path: &str) {
        let font_byte = read(path).unwrap();
        self.data.insert(String::from(name), font_byte);
    }

    pub fn font_byte(&self, name: &String) -> Option<&Vec<u8>> {
        self.data.get(name)
    }
}