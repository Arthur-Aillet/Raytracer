//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

use crate::vectors;

use vectors::Segment;
use vectors::Vector;
use vectors::resolve_quadratic_equation;
use super::renderer_common::{Transform, Texture};

pub struct Sphere {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
}

pub struct Plane {
    pub transform: Transform,
    pub texture: Texture,
    pub origin: Vector,
    pub vector: Vector,
}
pub struct Cylinder {
    pub transform: Transform,
    pub texture: Texture,
    pub height: f64,
    pub radius: f64,
}

pub struct Cone {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
    pub height: f64,
}

pub trait Object {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment>;
    fn obj_type(&self) -> String;
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
    fn get_texture(&self) -> Texture;
    fn set_texture(&mut self, new: Texture);
    fn get_radius(&self) -> f64;
    fn set_radius(&mut self, new: f64);
    fn get_height(&self) -> f64;
    fn set_height(&mut self, new: f64);
    fn get_origin(&self) -> Vector;
    fn set_origin(&mut self, new: Vector);
    fn get_vector(&self) -> Vector;
    fn set_vector(&mut self, new: Vector);
    fn slanted_height(&self) -> f64;
    fn diameter(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn surface(&self) -> f64;
    fn lateral_surface(&self) -> f64;
    fn base_surface(&self) -> f64;
    fn volume(&self) -> f64;
}

impl Object for Sphere {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {
        let diff = camera - self.transform.pos;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - self.radius.powi(2));

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|a, b| a.partial_cmp(b).unwrap());
        //filter neg
        if smallest_result == None {
            None
        } else {
            Some ( Segment {
                origin : self.transform.pos,
                end: Vector {
                    x: camera.x + ray.x * smallest_result.unwrap(),
                    y: camera.y + ray.y * smallest_result.unwrap(),
                    z: camera.z + ray.z * smallest_result.unwrap(),
                }
            })
        }
    }
    fn obj_type(&self) -> String {format!("sphere")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn diameter(&self) -> f64 {self.radius * 2.0}
    fn perimeter(&self) -> f64 {self.radius * std::f64::consts::PI * 2.0}
    fn surface(&self) -> f64 {self.radius.powi(2) * std::f64::consts::PI * 4.0}
    fn volume(&self) -> f64 {((std::f64::consts::PI * self.radius.powi(3)) * 4.0) / 3.0}

    fn get_height(&self) -> f64 {1.0}
    fn set_height(&mut self, _new: f64) {}
    fn get_origin(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Vector) {}
    fn get_vector(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Vector) {}
    fn slanted_height(&self) -> f64 {1.0}
    fn lateral_surface(&self) -> f64 {self.surface()}
    fn base_surface(&self) -> f64 {self.surface()}
}

impl Object for Plane {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {return None;}
    fn obj_type(&self) -> String {format!("plane")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_origin(&self) -> Vector {self.origin}
    fn set_origin(&mut self, new: Vector) {self.origin = new}
    fn get_vector(&self) -> Vector {self.vector}
    fn set_vector(&mut self, new: Vector) {self.vector = new}

    fn get_radius(&self) -> f64 {1.0}
    fn set_radius(&mut self, _new: f64) {}
    fn get_height(&self) -> f64 {1.0}
    fn set_height(&mut self, _new: f64) {}
    fn slanted_height(&self) -> f64 {1.0}
    fn diameter(&self) -> f64 {1.0}
    fn perimeter(&self) -> f64 {1.0}
    fn surface(&self) -> f64 {1.0}
    fn lateral_surface(&self) -> f64 {1.0}
    fn base_surface(&self) -> f64 {1.0}
    fn volume(&self) -> f64 {1.0}
}

impl Object for Cylinder {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {return None;}
    fn obj_type(&self) -> String {format!("cylinder")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn get_height(&self) -> f64 {self.height}
    fn set_height(&mut self, new: f64) {self.height = new}
    fn diameter(&self) -> f64 {self.radius * 2.0}
    fn perimeter(&self) -> f64 {self.radius * std::f64::consts::PI * 2.0}
    fn surface(&self) -> f64 {2.0 * self.base_surface() + self.lateral_surface()}
    fn lateral_surface(&self) -> f64 {2.0 * std::f64::consts::PI * self.radius * self.height}
    fn base_surface(&self) -> f64 {std::f64::consts::PI * self.radius.powi(2)}
    fn volume(&self) -> f64 {std::f64::consts::PI * self.radius.powi(2) * self.height}

    fn get_origin(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Vector) {}
    fn get_vector(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Vector) {}
    fn slanted_height(&self) -> f64 {1.0}
}

impl Object for Cone {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {return None;}
    fn obj_type(&self) -> String {format!("cone")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn get_height(&self) -> f64 {self.height}
    fn set_height(&mut self, new: f64) {self.height = new}
    fn slanted_height(&self) -> f64 {(self.radius.powi(2) + self.height.powi(2)).sqrt()}
    fn diameter(&self) -> f64 {self.radius * 2.0}
    fn perimeter(&self) -> f64 {self.radius * std::f64::consts::PI * 2.0}
    fn surface(&self) -> f64 {self.lateral_surface() + self.base_surface()}
    fn lateral_surface(&self) -> f64 {std::f64::consts::PI * self.radius * self.slanted_height()}
    fn base_surface(&self) -> f64 {std::f64::consts::PI * self.radius.powi(2)}
    fn volume(&self) -> f64 {(std::f64::consts::PI * self.radius.powi(2) * self.height) / 3.0}

    fn get_origin(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Vector) {}
    fn get_vector(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Vector) {}
}
