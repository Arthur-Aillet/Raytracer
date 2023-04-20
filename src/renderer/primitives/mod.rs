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

pub trait Object {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment>;
}

#[derive(Debug)]
pub struct Light {
    pub origin: Vector,
    pub intensity: f64,
}

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vector,
    pub radius: f64,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub shininess: f64,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Plan {
    origin: Vector,
    endPoint: Vector,
}

impl Sphere {
    pub fn set_origin(&mut self, origin: Vector) {
        self.origin = origin;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

impl Plan {
    pub fn set_origin(&mut self, point: Vector) {
        self.origin = point;
    }
    pub fn set_endPoint(&mut self, point: Vector) {
        self.endPoint = point;
    }
}

impl Object for Sphere {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {
        let diff = camera - self.origin;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                   2.0 * (ray.dot_product(diff)),
                                   (diff.dot_product(diff)) - self.radius.powi(2));

        let smallest_result: Option<&f64> = result.iter().min_by(|a, b| a.partial_cmp(b).unwrap());

        if smallest_result == None {
            None
        } else {
            Some ( Segment {
                origin : self.origin.clone(),
                end: Vector {
                    x: camera.x + ray.x * smallest_result.unwrap_or(&0.0),
                    y: camera.y + ray.y * smallest_result.unwrap_or(&0.0),
                    z: camera.z + ray.z * smallest_result.unwrap_or(&0.0),
                }
            })
        }
    }
}

impl Object for Plan {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {
        return None;
    }
}
