//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

use crate::vectors;
use crate::vectors::resolve_quadratic_equation;

pub trait Object {
    fn intersection(&self, ray: vectors::VectorF) -> bool;
}

#[derive(Debug)]
pub struct Sphere {
    pub origin: vectors::Point,
    pub radius: f64,
}

pub struct Plan {
    origin: vectors::Point,
    endPoint: vectors::Point,
}

impl Sphere {
    pub fn set_origin(&mut self, origin: vectors::Point) {
        self.origin = origin;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

impl Plan {
    pub fn set_origin(&mut self, point: vectors::Point) {
        self.origin = point;
    }
    pub fn set_endPoint(&mut self, point: vectors::Point) {
        self.endPoint = point;
    }
}

impl Object for Sphere {
    fn intersection(&self, ray: vectors::VectorF) -> bool {
        //resolve_quadratic_equation(ray.direction * ray.direction, 2 * ray.origin * ray.direction, ray.origin * ray.origin - radius * radius);
        if ray.direction.x > 50.0 {
            true
        } else {
            false
        }
    }
}

impl Object for Plan {
    fn intersection(&self, ray: vectors::VectorF) -> bool {
        return true;
    }
}
