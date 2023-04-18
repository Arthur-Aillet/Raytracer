//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

pub trait Object {
    fn intersection(&self, ray: VectorF) -> bool;
}

pub struct Sphere {
    origin: Point,
    radius: f64,
}

pub struct Plan {
    origin: Point,
    endPoint: Point,
}

impl Sphere {
    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

impl Plan {
    pub fn set_origin(&mut self, point: Point) {
        self.origin = point;
    }
    pub fn set_endPoint(&mut self, point: Point) {
        self.endPoint = point;
    }
}

impl Object for Sphere {
    fn intersection(&self, ray: VectorF) -> bool {
        return true;
    }
}

impl Object for Plan {
    fn intersection(&self, ray: VectorF) -> bool {
        return true;
    }
}
