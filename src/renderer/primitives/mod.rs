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

#[derive(Debug)]
pub struct Plan {
    pub normal: Vector,
    pub distance: f64,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub shininess: f64,
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
    pub fn set_normal(&mut self, point: Vector) {
        self.normal = point;
    }
    pub fn set_distance(&mut self, distance: f64) {
        self.distance = distance;
    }
}

impl Object for Sphere {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {
        let diff = camera - self.origin;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - self.radius.powi(2));

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|a, b| a.partial_cmp(b).unwrap());
        //filter neg
        if smallest_result == None {
            None
        } else {
            Some ( Segment {
                origin : self.origin.clone(),
                end: Vector {
                    x: camera.x + ray.x * smallest_result.unwrap(),
                    y: camera.y + ray.y * smallest_result.unwrap(),
                    z: camera.z + ray.z * smallest_result.unwrap(),
                }
            })
        }
    }
}

impl Object for Plan {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {
        //R0 = camera
        //Rd = ray_dest
        // Pn = plan_norm normal du plan (normalisée)
        let ray_dest = ray.normalize();
        let plan_norm = self.normal.normalize();
        let vd = ray_dest.dot_product(plan_norm);

        if vd == 0.0 { // ray parallel to the plan
            return None;
        }

        let v0 = -(plan_norm.dot_product(camera) + self.distance);
        if v0 < 0.0 { // intersection behind the camera
            return None;
        }

        let t = v0 / (plan_norm.dot_product(ray_dest));
        let intersection_point = Vector{
            x: camera.x + ray_dest.x * t,
            y: camera.y + ray_dest.y * t,
            z: camera.z + ray_dest.z * t
        };
        Some ( Segment {
            origin: intersection_point,
            end: Vector{
                x: intersection_point.x + plan_norm.x,
                y: intersection_point.y + plan_norm.y,
                z: intersection_point.z + plan_norm.z,
            }
        })
    }
}
