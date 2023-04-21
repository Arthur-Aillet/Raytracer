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
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub intersection_point: Vector,
    pub normal: Vector,
    pub object: Sphere,
}

#[derive(Debug)]
pub struct Light {
    pub origin: Vector,
    pub intensity: f64,
    pub color: Vector,
    pub radius: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Vector,
    pub radius: f64,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub shininess: f64,
    pub color: Vector,
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
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.origin;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - self.radius.powi(2));

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|a, b| a.partial_cmp(b).unwrap());
        //filter neg
        if smallest_result == None {
            None
        } else {
            let point = Vector {
                x: origin.x + ray.x * smallest_result.unwrap(),
                y: origin.y + ray.y * smallest_result.unwrap(),
                z: origin.z + ray.z * smallest_result.unwrap(),
            };
            Some ( Intersection {
                normal: point - self.origin,
                intersection_point: point,
                object: *self
            })
        }
    }
}

impl Object for Plan {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        return None;
    }
}
