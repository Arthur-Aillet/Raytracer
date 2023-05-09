//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// lights
//

use super::camera::Camera;
use super::renderer_common::{Transform, Color};
use super::primitives::{Intersection, Object};
use crate::vectors;
use vectors::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub transform: Transform,
    pub color: Color,
    pub strength: f64,
    pub radius: f64,
    pub falloff: i32,
}

pub struct Directional {
    pub transform: Transform,
    pub color: Color,
    pub strength: f64,

}

pub trait Light {
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
    fn get_color(&self) -> Color;
    fn set_color(&mut self, new: Color);
    fn get_strength(&self) -> f64;
    fn set_strength(&mut self, new: f64);
    fn get_radius(&self) -> f64;
    fn set_radius(&mut self, new: f64);
    fn get_falloff(&self) -> i32;
    fn set_falloff(&mut self, new: i32);
    fn light_is_intersected(&self, light_vector: Vector, intersect: &Intersection, normal_vector: Vector, camera: Camera, primitives: &Vec<Box<dyn Object + Send + Sync>>) -> bool;
    fn calculate_light(&self, intersect: &Intersection, camera_to_pixel: Vector, camera: Camera, primitives: &Vec<Box<dyn Object + Send + Sync>>) -> Vector;
}

impl Light for Point {
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_color(&self) -> Color {self.color}
    fn set_color(&mut self, new: Color) {self.color = new}
    fn get_strength(&self) -> f64 {self.strength}
    fn set_strength(&mut self, new: f64) {self.strength = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn get_falloff(&self) -> i32 {self.falloff}
    fn set_falloff(&mut self, new: i32) {self.falloff = new}
    fn light_is_intersected(&self, light_vector: Vector, intersect: &Intersection, normal_vector: Vector, camera: Camera, primitives: &Vec<Box<dyn Object + Send + Sync>>) -> bool {
        for object_current in primitives.iter() {
            match object_current.intersection(light_vector, intersect.intersection_point + (normal_vector * camera.shadow_bias)) {
                None => { continue }
                Some(shadow_intersect) => {
                    if (shadow_intersect.intersection_point - intersect.intersection_point).len() < (self.transform.pos - intersect.intersection_point).len() {
                        return true
                    }
                }
            }
        }
        false
    }
    fn calculate_light(&self, intersect: &Intersection, camera_to_pixel: Vector, camera: Camera, primitives: &Vec<Box<dyn Object + Send + Sync>>) -> Vector {
        let normal_vector = intersect.normal.normalize();
        let light_vector = (self.transform.pos - intersect.intersection_point).normalize();
        let mut light_uncovered = 1.0;

        if camera.smooth_shadow == false {
            if self.light_is_intersected(light_vector, intersect, normal_vector, camera, primitives) {
                return Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }
        } else {
            let mut light_reached: i16 = 0;
            for _ in 0..camera.smooth_shadow_step {
                let inter_to_light = self.transform.pos + Vector::get_random_point_in_sphere(self.radius) - intersect.intersection_point;
                if self.light_is_intersected(inter_to_light.normalize(), intersect, normal_vector, camera, primitives) == false {
                    light_reached += 1;
                }
            }
            light_uncovered = light_reached as f64 / camera.smooth_shadow_step as f64;
        }
        let diffuse = light_vector.dot_product(normal_vector).max(0.0) * camera.diffuse * intersect.object.get_texture().diffuse;

        let reflected = light_vector.reflect(normal_vector).normalize();
        let view = (camera_to_pixel * -1.0).normalize();
        let specular = camera.specular * intersect.object.get_texture().specular * reflected.dot_product(view).max(0.0).powf(intersect.object.get_texture().shininess);
        let distance = intersect.intersection_point.distance(self.transform.pos);
        let light_falloff = (self.strength/ distance.powi(self.falloff)).max(0.0);
        intersect.object.get_texture().color.as_vector() * self.color.as_vector() * diffuse * light_falloff * light_uncovered + self.color.as_vector() * specular * light_falloff * light_uncovered
    }
}

impl Light for Directional {
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_color(&self) -> Color {self.color}
    fn set_color(&mut self, new: Color) {self.color = new}
    fn get_strength(&self) -> f64 {self.strength}
    fn set_strength(&mut self, new: f64) {self.strength = new}
    fn get_radius(&self) -> f64 {1.0}
    fn set_radius(&mut self, _new: f64) {}
    fn get_falloff(&self) -> i32 {0}
    fn set_falloff(&mut self, _new: i32) {}
    fn light_is_intersected(&self, light_vector: Vector, intersect: &Intersection, normal_vector: Vector, camera: Camera, primitives: &Vec<Box<dyn Object + Send + Sync>>) -> bool {
        for object_current in primitives.iter() {
            match object_current.intersection(light_vector, intersect.intersection_point + (normal_vector * camera.shadow_bias)) {
                None => { continue }
                Some(shadow_intersect) => {
                    if (shadow_intersect.intersection_point - intersect.intersection_point).len() < (self.transform.pos - intersect.intersection_point).len() {
                        return true
                    }
                }
            }
        }
        false
    }
    fn calculate_light(&self, intersect: &Intersection, camera_to_pixel: Vector, camera: Camera, primitives: &Vec<Box<dyn Object + Send + Sync>>) -> Vector {
        let normal_vector = intersect.normal.normalize();
        let mut light_uncovered = 1.0;

        if camera.smooth_shadow == false {
            if self.light_is_intersected(self.transform.pos, intersect, normal_vector, camera, primitives) {
                return Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }
        } else {
            let mut light_reached: i16 = 0;
            for _ in 0..camera.smooth_shadow_step {
                let inter_to_light = self.transform.pos + Vector::get_random_point_in_sphere(0.0) - intersect.intersection_point;
                if self.light_is_intersected(inter_to_light.normalize(), intersect, normal_vector, camera, primitives) == false {
                    light_reached += 1;
                }
            }
            light_uncovered = light_reached as f64 / camera.smooth_shadow_step as f64;
        }
        let diffuse = self.transform.pos.dot_product(normal_vector).max(0.0) * camera.diffuse * intersect.object.get_texture().diffuse;

        let reflected = self.transform.pos.reflect(normal_vector).normalize();
        let view = (camera_to_pixel * -1.0).normalize();
        let specular = camera.specular * intersect.object.get_texture().specular * reflected.dot_product(view).max(0.0).powf(intersect.object.get_texture().shininess);
        intersect.object.get_texture().color.as_vector() * self.color.as_vector() * diffuse * light_uncovered + self.color.as_vector() * specular * light_uncovered
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
