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

pub struct Intersection<'a> {
    pub intersection_point: Vector,
    pub normal: Vector,
    pub object: Option<&'a dyn Object>,
    pub light: Option<&'a dyn Light>,
}

pub struct Sphere {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
}

pub struct Plane {
    pub transform: Transform,
    pub texture: Texture,
    pub normal: Vector,
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
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
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
                                                (diff.dot_product(diff)) - self.radius.powi(2));

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
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, _new: Vector) {}
}

impl Object for Plane {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let normal = self.normal.normalize();
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
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, _new: f64) {}

    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, new: Vector) {self.normal = new}
}

impl Object for Cylinder {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        /*
        base == C = Point at the center of the base of the cylinder
        top == H = Point at the center of the top of the cylinder
        radius == r = Cylinder radius
        P = Point on the cylinder surface

        origin == L0 = Point on the line
        ray == v = Vector that defines the line direction
        axis == ĥ
        */

        let mut axis = Vector{
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        axis.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        axis = axis.normalize();
        let distance = origin - (self.transform.pos - axis * (self.height / 2.0));

        //println!("{:?}", distance);
        let a = ray.dot_product(ray) - (ray.dot_product(axis)).powi(2);
        let b = 2.0 * (ray.dot_product(distance) - ray.dot_product(axis) * distance.dot_product(axis));
        let c = distance.dot_product(distance) - distance.dot_product(axis).powi(2) - self.radius.powi(2);

        if b.powi(2) - 4.0 * a * c > 0.0 {
            let result = resolve_quadratic_equation(a, b, c);

            let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|fst, snd| fst.partial_cmp(snd).unwrap());
            if smallest_result == None {
                return None;
            }

            return Some ( Intersection {
                intersection_point: Vector{
                    x: origin.x + ray.x * smallest_result.unwrap(),
                    y: origin.y + ray.y * smallest_result.unwrap(),
                    z: origin.z + ray.z * smallest_result.unwrap()
                },
                normal: Vector{
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                object: Some(self),
                light: None,
            })
        }
        if b.powi(2) - 4.0 * a * c == 0.0 {
            let progress = - (b / 2.0 * a);

            return Some ( Intersection {
                intersection_point: Vector{
                    x: origin.x + ray.x * progress,
                    y: origin.y + ray.y * progress,
                    z: origin.z + ray.z * progress
                },
                normal: Vector{
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                object: Some(self),
                light: None,
            })
        }
        None
    }
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, new: f64) {self.height = new}
    fn set_normal(&mut self, _new: Vector) {}
}

impl Object for Cone {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {return None;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, new: f64) {self.height = new}
    fn set_normal(&mut self, _new: Vector) {}
}
