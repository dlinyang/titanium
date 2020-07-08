use rmu::raw::*;

#[derive(Debug,Clone,PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    Float(f32),
    Vec2(Vec2f),
    Vec3(Vec3f),
    Vec4(Vec4f),
    Texture(String),
}

#[derive(Clone)]
pub struct Material {
    pub name: String,
    pub property: Vec<(String,PropertyValue)>,
}

impl Material {
    pub fn material_name(&self) -> String {
        self.name.clone()
    }

    pub fn property(&self) -> Vec<(String,PropertyValue)> {
        self.property.clone()
    }
}

pub fn color_canvas(color: Vec4f) -> Material {
    let name: String = "Color Canvas".into();
    let property: Vec<(String, PropertyValue)> = vec![("color".into(), PropertyValue::Vec4(color))];

    Material {
        name,
        property,
    }
}

pub fn image_canvas(color: Vec4f) -> Material {
    let name: String = "Image Canvas".into();
    let property: Vec<(String, PropertyValue)> = vec![("color".into(), PropertyValue::Vec4(color))];

    Material {
        name,
        property,
    }
}

pub fn font_canvas(color: Vec4f) -> Material {
    let name: String = "Font Canvas".into();
    let property: Vec<(String, PropertyValue)> = vec![("color".into(), PropertyValue::Vec4(color))];

    Material {
        name,
        property,
    }
}

///
pub fn pure_color_material(color: Vec3f) -> Material {
    let name: String = "Pure Color Material".into();
    let property: Vec<(String, PropertyValue)> = vec![("material.color".into(), PropertyValue::Vec3(color))];

    Material {
        name,
        property,
    }
}

///bultin material for easy to use
pub fn blinn_phong_brdf(ambient: Vec3f, diffuse: Vec3f, specular: Vec3f, shininess: f32) -> Material { 
    let name: String = "Blinn Phong BRDF".into();

    let property: Vec<(String,PropertyValue)> = vec![
        ("material.ambient".into()  , PropertyValue::Vec3(ambient)),
        ("material.diffuse".into()  , PropertyValue::Vec3(diffuse)),
        ("material.shininess".into(), PropertyValue::Float(shininess)),
        ("material.specular".into() , PropertyValue::Vec3(specular))
    ];

    Material {
        name,
        property,
    }
}

pub fn cook_torrance_brdf<'a>(albedo: Vec3f, roughness: f32, metallic: f32, ao: f32) -> Material {
    let name: String =  "Cook Torrance BRDF".into();

    let property: Vec<(String,PropertyValue)> = vec![
        ("material.ambient".into()  , PropertyValue::Vec3(albedo)),
        ("material.roughness".into(), PropertyValue::Float(roughness)),
        ("material.metallic".into() , PropertyValue::Float(metallic)),
        ("material.ao".into()       , PropertyValue::Float(ao))
    ];

    Material {
        name,
        property,
    }
}