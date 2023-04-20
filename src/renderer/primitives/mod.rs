//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

use crate::vectors;
use vectors::VectorF;
use vectors::Point;
use vectors::resolve_quadratic_equation;

pub trait Object {
    fn intersection(&self, ray: Point, camera: Point) -> Option<VectorF>;
}

#[derive(Debug)]
pub struct Light {
    pub origin: Point,
    pub intensity: f64,
}

#[derive(Debug)]
pub struct Sphere {
    pub origin: Point,
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
    fn intersection(&self, ray: Point, camera: Point) -> Option<VectorF> {
        let diff = camera - self.origin;
        let result = resolve_quadratic_equation(ray.dot_product(ray),
                                   2.0 * (ray.dot_product(diff)),
                                   (diff.dot_product(diff)) - self.radius.powi(2));

        let mut smallest_result: Option<f64> = None;

        for num in result {
            if num > 0.0 {
                if smallest_result.is_none() || num < smallest_result.unwrap() {
                    smallest_result = Some(num);
                }
            }
        }
        if smallest_result == None {
            None
        } else {
            Some ( VectorF {
                origin : Point {
                    x: self.origin.x,
                    y: self.origin.y,
                    z: self.origin.z,
                },
                direction: Point {
                    x: camera.x + ray.x * smallest_result.unwrap_or(0.0),
                    y: camera.y + ray.y * smallest_result.unwrap_or(0.0),
                    z: camera.z + ray.z * smallest_result.unwrap_or(0.0),
                }
            })
        }
    }
}

impl Object for Plan {
    fn intersection(&self, ray: Point, camera: Point) -> Option<VectorF> {
        return None;
    }
}
