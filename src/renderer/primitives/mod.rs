//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

use crate::vectors;

pub trait Object {
    fn intersection(&self, ray: vectors::VectorF) -> bool;
}

pub struct Sphere {
    origin: vectors::Point,
    radius: f64,
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
        return true;
    }
}

impl Object for Plan {
    fn intersection(&self, ray: vectors::VectorF) -> bool {
        return true;
    }
}
