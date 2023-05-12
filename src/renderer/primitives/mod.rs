//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

pub mod mesh;

use crate::vectors;
use vectors::Vector;
use vectors::resolve_quadratic_equation;
use crate::renderer::lights::Light;
use super::renderer_common::{Transform, Texture};
use serde::{Deserialize, Serialize};
use erased_serde::serialize_trait_object;

pub struct Intersection<'a> {
    pub intersection_point: Vector,
    pub normal: Vector,
    pub object: Option<&'a dyn Object>,
    pub light: Option<&'a dyn Light>,
}

#[derive(Deserialize, Serialize)]
pub struct Sphere {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Plane {
    pub transform: Transform,
    pub texture: Texture,
    pub normal: Vector,
}

#[derive(Deserialize, Serialize)]
pub struct Cylinder {
    pub transform: Transform,
    pub texture: Texture,
    pub height: f64,
    pub radius: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Cone {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
    pub height: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Triangle {
    pub transform : Transform,
    pub texture: Texture,
    pub point_a: Vector,
    pub point_b: Vector,
    pub point_c: Vector
}

pub trait Object: erased_serde::Serialize {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
    fn surface_position(&self, position: Vector) -> Vector;
    fn get_transform(&self) -> Transform;
    fn move_obj(&mut self, offset: Transform);
    fn set_transform(&mut self, new: Transform);
    fn get_texture(&self) -> Texture;
    fn set_texture(&mut self, new: Texture);
}

impl Object for Sphere {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - (self.radius * self.transform.scale.x).powi(2));

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

    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: 2.0 * (1.0 - (rotated_position.x.atan2(rotated_position.y)/ (2.0 * std::f64::consts::PI) + 0.5)),
            y: 1.0 - (rotated_position.z / (rotated_position.x.powi(2) + rotated_position.y.powi(2) + rotated_position.z.powi(2)).sqrt()).acos() / std::f64::consts::PI,
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
}

impl Object for Plane {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut normal = self.normal.normalize();
        normal.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
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
    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: if position.x < 0.0 {position.x % 1.0 + 1.0} else {position.x % 1.0},
            y: if position.y < 0.0 {position.y % 1.0 + 1.0} else {position.y % 1.0},
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
}

impl Cone {
    fn base_intersection(&self, ray: Vector, origin: Vector, normal: Vector, center: Vector) -> Option<Intersection> {
        let normal = normal.normalize();
        let denom = ray.normalize().dot_product(normal);
        if denom == 0.0 {
            return None
        }
        let progress = (center - origin).dot_product(normal) / denom;
        if progress < 0.0 {
            return None
        }
        let intersection_point = Vector{
            x: origin.x + ray.x * progress,
            y: origin.y + ray.y * progress,
            z: origin.z + ray.z * progress
        };

        if (intersection_point - center).len() > self.radius {
            return None;
        }
        Some ( Intersection {
            intersection_point,
            normal,
            object: Some(self),
            light: None,
        })
    }
}

impl Cylinder {
    fn base_intersection(&self, ray: Vector, origin: Vector, normal: Vector, center: Vector) -> Option<Intersection> {
        let normal = normal.normalize();
        let denom = ray.normalize().dot_product(normal);
        if denom == 0.0 {
            return None
        }
        let progress = (center - origin).dot_product(normal) / denom;
        if progress < 0.0 {
            return None
        }
        let intersection_point = Vector{
            x: origin.x + ray.x * progress,
            y: origin.y + ray.y * progress,
            z: origin.z + ray.z * progress
        };

        if (intersection_point - center).len() > self.radius {
            return None;
        }
        Some ( Intersection {
            intersection_point,
            normal,
            object: Some(self),
            light: None,
        })
    }
}

impl Object for Cylinder {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut axis = Vector{
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        axis.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z); // Ĥ
        let base = self.transform.pos - axis * (self.height / 2.0);
        let top = self.transform.pos + axis * (self.height / 2.0);// C

        let distance = origin - base; // W
        // ray == V

        let a = 1.0 /*ray.dot_product(ray) car normalisé */ - (ray.dot_product(axis)).powi(2);
        let b = 2.0 * (ray.dot_product(distance) - ray.dot_product(axis) * distance.dot_product(axis));
        let c = distance.dot_product(distance) - distance.dot_product(axis).powi(2) - (self.radius.powi(2) / self.height.powi(2));

        let result = resolve_quadratic_equation(a, b, c);

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|fst, snd| fst.partial_cmp(snd).unwrap());
        if smallest_result == None { return None; }

        let intersection_point = origin + ray * *smallest_result.unwrap();

        if -self.height / 2.0 <= (intersection_point - self.transform.pos).dot_product(axis) && (intersection_point - self.transform.pos).dot_product(axis) <= self.height / 2.0 { // too far from center
            let normal = intersection_point - (base + axis * (intersection_point - base).dot_product(axis)); // Cos(teta) = A/H

            return Some ( Intersection {
                intersection_point,
                normal,
                object: Some(self),
                light: None,
            })
        }
        if (intersection_point - base).dot_product(axis) < 0.0 {
            return self.base_intersection(ray, origin, axis * -1.0, base);
        }
        if (intersection_point - base).dot_product(axis) > self.height {
            return self.base_intersection(ray, origin,  axis, top);
        }
        None
    }
    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: 1.0 - (rotated_position.x.atan2(rotated_position.y) / (2.0 * std::f64::consts::PI) + 0.5),
            y: rotated_position.z % 1.0,
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}

}

impl Object for Cone {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut axis = Vector{
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        axis.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z); // Ĥ
        let base = self.transform.pos - axis * (self.height / 2.0);
        let top = self.transform.pos + axis * (self.height / 2.0);// C

        let distance = origin - top; // W
        // ray == V

        let radius_constant = self.radius.powi(2) / self.height.powi(2);

        let a = 1.0 - radius_constant * (ray.dot_product(axis)).powi(2) - (ray.dot_product(axis)).powi(2);
        let b = 2.0 * (ray.dot_product(distance) - radius_constant * ray.dot_product(axis) * distance.dot_product(axis) - ray.dot_product(axis) * distance.dot_product(axis));
        let c = distance.dot_product(distance) - radius_constant * distance.dot_product(axis).powi(2) - distance.dot_product(axis).powi(2);

        let result = resolve_quadratic_equation(a, b, c);

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|fst, snd| fst.partial_cmp(snd).unwrap());
        if smallest_result == None { return None; }

        let intersection_point = origin + ray * *smallest_result.unwrap();
        if 0.0 <= (intersection_point - base).dot_product(axis) && (intersection_point - base).dot_product(axis) <= self.height { // too far from center*/
            let cos_angle = axis.dot_product(top - intersection_point);
            let normal = (intersection_point - (top - axis * ((top - intersection_point).len2() / cos_angle))).normalize();

            return Some ( Intersection {
                intersection_point,
                normal,
                object: Some(self),
                light: None,
            });
        }
        if (intersection_point - top).dot_product(axis) < 0.0 {
            return self.base_intersection(ray, origin, axis * -1.0, base);
        }
        None
    }
    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: 1.0 - (rotated_position.x.atan2(rotated_position.y) / (2.0 * std::f64::consts::PI) + 0.5),
            y: rotated_position.z % 1.0,
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
}

impl Object for Triangle {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut point_a = self.point_a.clone();
        point_a.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        point_a = point_a + self.transform.pos;
        let mut point_b = self.point_b.clone();
        point_b.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        point_b = point_b + self.transform.pos;
        let mut point_c = self.point_c.clone();
        point_c.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        point_c = point_c + self.transform.pos;

        let mut normal = (point_b - point_a).cross_product(point_c - point_a).normalize();

        let denom = ray.normalize().dot_product(normal);
        if denom == 0.0 {
            return None
        }
        let progress = (((point_a + point_b + point_c) / 3.0) - origin).dot_product(normal) / denom;
        if progress < 0.0 {
            return None
        }
        let intersection_point = Vector{
            x: origin.x + ray.x * progress,
            y: origin.y + ray.y * progress,
            z: origin.z + ray.z * progress
        };

        let cross = (point_b - point_a).cross_product(intersection_point - point_a);
        if normal.dot_product(cross) < 0.0 {
            return None;
        }

        let cross = (point_c - point_b).cross_product(intersection_point - point_b);
        if normal.dot_product(cross) < 0.0 {
            return None;
        }

        let cross = (point_a - point_c).cross_product(intersection_point - point_c);
        if normal.dot_product(cross) < 0.0 {
            return None;
        }

        if normal.dot_product(origin - intersection_point) < 0.0 {
            normal = normal * -1.0;
        }

        Some ( Intersection {
            intersection_point,
            normal,
            object: Some(self),
            light: None,
        })
    }
    fn surface_position(&self, position: Vector) -> Vector {Vector { x: 0.5, y: 0.5, z: 0.0}}
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
}

serialize_trait_object!(Object);
