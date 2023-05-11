//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

use crate::vectors;

use vectors::Vector;
use vectors::resolve_quadratic_equation;
use crate::renderer::lights::Light;
use super::renderer_common::{Transform, Texture};
use serde::{Deserialize, Serialize};
use erased_serde::serialize_trait_object;

pub struct Intersection<'a> {
    pub intersection_point: Vector,
    pub normal: Vector,
    pub object: Option<&'a dyn Object>,
    pub light: Option<&'a dyn Light>,
}

#[derive(Deserialize, Serialize)]
pub struct Sphere {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Plane {
    pub transform: Transform,
    pub texture: Texture,
    pub normal: Vector,
}

#[derive(Deserialize, Serialize)]
pub struct Cylinder {
    pub transform: Transform,
    pub texture: Texture,
    pub height: f64,
    pub radius: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Cone {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
    pub height: f64,
}

pub trait Object: erased_serde::Serialize {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
    fn surface_position(&self, position: Vector) -> Vector;
    fn get_transform(&self) -> Transform;
    fn move_obj(&mut self, offset: Transform);
    fn set_transform(&mut self, new: Transform);
    fn get_texture(&self) -> Texture;
    fn set_texture(&mut self, new: Texture);
    fn set_radius(&mut self, new: f64);
    fn set_height(&mut self, new: f64);
    fn set_normal(&mut self, new: Vector);
}

impl Object for Sphere {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - (self.radius * self.transform.scale.x).powi(2));

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
                object: Some(self),
                light: None
            })
        }
    }

    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: 2.0 * (1.0 - (rotated_position.x.atan2(rotated_position.y)/ (2.0 * std::f64::consts::PI) + 0.5)),
            y: 1.0 - (rotated_position.z / (rotated_position.x.powi(2) + rotated_position.y.powi(2) + rotated_position.z.powi(2)).sqrt()).acos() / std::f64::consts::PI,
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, _new: Vector) {}
}

impl Object for Plane {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut normal = self.normal.normalize();
        normal.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        let denom = ray.normalize().dot_product(normal);
        if denom == 0.0 {
            return None
        }
        let progress = (self.transform.pos - origin).dot_product(normal) / denom;
        if progress < 0.0 {
            return None
        }
        Some ( Intersection {
            intersection_point: Vector{
                x: origin.x + ray.x * progress,
                y: origin.y + ray.y * progress,
                z: origin.z + ray.z * progress
            },
            normal,
            object: Some(self),
            light: None,
        })
    }
    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: {position.x % 1.0},
            y: {position.y % 1.0},
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, _new: f64) {}

    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, new: Vector) {self.normal = new}
}

impl Object for Cylinder {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {return None;}
    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: 1.0 - (rotated_position.x.atan2(rotated_position.y) / (2.0 * std::f64::consts::PI) + 0.5),
            y: rotated_position.z % 1.0,
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, new: f64) {self.height = new}
    fn set_normal(&mut self, _new: Vector) {}
}

impl Object for Cone {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {return None;}
    fn surface_position(&self, position: Vector) -> Vector {Vector { x: 0.5, y: 0.5, z: 0.0}}
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, new: f64) {self.height = new}
    fn set_normal(&mut self, _new: Vector) {}
}

serialize_trait_object!(Object);
