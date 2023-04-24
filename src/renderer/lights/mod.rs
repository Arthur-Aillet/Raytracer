//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// lights
//

use super::renderer_common::{Transform, Color};


#[derive(Debug, Clone, Copy)]
pub struct Directional {
    pub transform: Transform,
    pub color: Color,
    pub strength: f64,
    pub radius: f64,
}

pub trait Light {
    fn light_type(&self) -> String;
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
    fn get_color(&self) -> Color;
    fn set_color(&mut self, new: Color);
    fn get_strength(&self) -> f64;
    fn set_strength(&mut self, new: f64);
    fn get_radius(&self) -> f64;
    fn set_radius(&mut self, new: f64);
}

impl Light for Directional {
    fn light_type(&self) -> String {format!("directional")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_color(&self) -> Color {self.color}
    fn set_color(&mut self, new: Color) {self.color = new}
    fn get_strength(&self) -> f64 {self.strength}
    fn set_strength(&mut self, new: f64) {self.strength = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
}

pub struct Ambient {
    pub color: Color,
    pub strength: f64,
}

impl Ambient {
    pub fn default() -> Ambient {
        Ambient {
            color: Color::default(),
            strength: 1000.0,
        }
    }
}

pub struct Lights {
    pub lights: Vec::<Box::<dyn Light>>,
    pub ambient: Vec<Ambient>,
}

impl Lights {
    pub fn default() -> Lights {
        let mut lights = Lights {
            lights: Vec::new(),
            ambient: Vec::new(),
        };
        lights.ambient.push(Ambient::default());
        lights
    }
}
