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
    fn intersection(&self, ray: VectorF) -> Option<VectorF>;
}

#[derive(Debug)]
pub struct Light {
    pub origin: Point,
}

#[derive(Debug)]
pub struct Sphere {
    pub origin: Point,
    pub radius: f64,
}

pub struct Plan {
    pub normal: VectorF,
    pub distance: f64
}

impl Sphere {
    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
}

// impl Plan {
//     pub fn set_origin(&mut self, point: Point) {
//         self.origin = point;
//     }
//     pub fn set_endPoint(&mut self, point: Point) {
//         self.endPoint = point;
//     }
// }

impl Object for Sphere {
    fn intersection(&self, ray: VectorF) -> Option<VectorF> {
        let result = resolve_quadratic_equation(ray.direction.x.powf(2.0) + ray.direction.y.powf(2.0) + ray.direction.z.powf(2.0),
            2.0 * (ray.direction.x * (ray.origin.x - self.origin.x) + ray.direction.y * (ray.origin.y - self.origin.y) + ray.direction.z * (ray.origin.z - self.origin.z)),
            ((ray.origin.x - self.origin.x).powf(2.0) + (ray.origin.y - self.origin.y).powf(2.0) + (ray.origin.z - self.origin.z).powf(2.0)) - self.radius.powf(2.0));

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
                    x: ray.origin.x + ray.direction.x * smallest_result.unwrap_or(0.0),
                    y: ray.origin.y + ray.direction.y * smallest_result.unwrap_or(0.0),
                    z: ray.origin.z + ray.direction.z * smallest_result.unwrap_or(0.0),
                }
            })
        }
    }
}

impl Object for Plan {
    fn intersection(&self, ray: VectorF) -> Option<VectorF> {
        //R0 = ray.origin
        //Rd = ray_dest
        // Pn = plan_norm.direction normal du plan (normalisée)
        let ray_dest = ray.normalize().direction;
        let plan_norm = self.normal.normalize();
        let vd = ray_dest.dot_product(&plan_norm.direction);

        if vd == 0.0 { // ray parallel to the plan
            return None;
        }

        let v0 = -(plan_norm.direction.dot_product(&ray.origin) + self.distance);
        if v0 < 0.0 { // intersection behind the camera
            return None;
        }

        let t = v0 / (plan_norm.direction.dot_product(&ray_dest));
        let intersection_point = Point{
            x: ray.origin.x + ray_dest.x * t,
            y: ray.origin.y + ray_dest.y * t,
            z: ray.origin.z + ray_dest.z * t
        };
        Some ( VectorF {
            origin: intersection_point,
            direction: Point {
                x: intersection_point.x + plan_norm.to_origin().direction.x,
                y: intersection_point.y + plan_norm.to_origin().direction.y,
                z: intersection_point.z + plan_norm.to_origin().direction.z,
            }
        })
    }
}
