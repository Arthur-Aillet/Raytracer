//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// lights
//

use crate::renderer::primitives::Intersection;
use crate::vectors::{resolve_quadratic_equation, Vector};
use super::renderer_common::{Transform, Color};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub transform: Transform,
    pub color: Color,
    pub strength: f64,
    pub radius: f64,
    pub falloff: i32,
    pub visible: bool,
}

pub trait Light {
    fn light_type(&self) -> String;
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
    fn get_color(&self) -> Color;
    fn get_visible(&self) -> bool;
    fn set_color(&mut self, new: Color);
    fn get_strength(&self) -> f64;
    fn set_strength(&mut self, new: f64);
    fn get_radius(&self) -> f64;
    fn set_radius(&mut self, new: f64);
    fn get_falloff(&self) -> i32;
    fn set_falloff(&mut self, new: i32);
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
}

impl Light for Point {
    fn light_type(&self) -> String {format!("point")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_color(&self) -> Color {self.color}
    fn get_visible(&self) -> bool {self.visible}
    fn set_color(&mut self, new: Color) {self.color = new}
    fn get_strength(&self) -> f64 {self.strength}
    fn set_strength(&mut self, new: f64) {self.strength = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn get_falloff(&self) -> i32 {self.falloff}
    fn set_falloff(&mut self, new: i32) {self.falloff = new}
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - self.radius.powi(2));

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|a, b| a.partial_cmp(b).unwrap());

        if smallest_result == None {
            None
        } else {
            let point = Vector {
                x: origin.x + ray.x * smallest_result.unwrap(),
                y: origin.y + ray.y * smallest_result.unwrap(),
                z: origin.z + ray.z * smallest_result.unwrap(),
            };
            Some ( Intersection {
                normal: point - self.transform.pos,
                intersection_point: point,
                object: None,
                light: Some(self)
            })
        }
    }
}

pub struct Ambient {
    pub color: Color,
    pub strength: f64,
}

impl Ambient {
    pub fn default() -> Ambient {
        Ambient {
            color: Color::default(),
            strength: 80.0,
        }
    }
}

pub struct Lights {
    pub lights: Vec::<Box::<dyn Light + Send + Sync>>,
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
