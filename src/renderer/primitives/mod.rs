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
    pub(crate) radius_applied: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Plane {
    pub transform: Transform,
    pub texture: Texture,
    pub normal: Vector,
    pub(crate) normal_applied: Vector,
}

#[derive(Deserialize, Serialize)]
pub struct Cylinder {
    pub transform: Transform,
    pub texture: Texture,
    pub height: f64,
    pub radius: f64,
    pub(crate) axis: Vector,
    pub(crate) top: Vector,
    pub(crate) base: Vector,
    pub(crate) radius_applied: f64,
    pub(crate) height_applied: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Cone {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
    pub height: f64,
    pub(crate) axis: Vector,
    pub(crate) top: Vector,
    pub(crate) base: Vector,
    pub(crate) radius_applied: f64,
    pub(crate) height_applied: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Triangle {
    pub transform : Transform,
    pub texture: Texture,
    pub point_a: Vector,
    pub point_b: Vector,
    pub point_c: Vector,
    pub(crate) point_a_applied: Vector,
    pub(crate) point_b_applied: Vector,
    pub(crate) point_c_applied: Vector,
    pub(crate) normal: Vector
}

#[derive(Deserialize, Serialize)]
pub struct Mesh {
    pub transform: Transform,
    pub texture: Texture,
    pub triangles: Vec<Triangle>,
}

pub trait Object: erased_serde::Serialize {
    fn apply_transform(&mut self);
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
    fn surface_position(&self, position: Vector) -> Vector;
    fn get_transform(&self) -> Transform;
    fn move_obj(&mut self, offset: Transform);
    fn set_transform(&mut self, new: Transform);
    fn get_texture(&self) -> Texture;
    fn set_texture(&mut self, new: Texture);
}

impl Object for Sphere {
    fn apply_transform(&mut self) {
        self.radius_applied = self.radius * self.transform.scale;
    }

    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - self.radius_applied.powi(2));

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
                normal: (point - self.transform.pos).normalize(),
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
    fn apply_transform(&mut self) {
        self.normal_applied = self.normal.normalize();
        self.normal_applied.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
    }

    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let denom = ray.normalize().dot_product(self.normal_applied);
        if denom == 0.0 {
            return None
        }
        let progress = (self.transform.pos - origin).dot_product(self.normal_applied) / denom;
        if progress < 0.0 {
            return None
        }
        Some ( Intersection {
            intersection_point: Vector{
                x: origin.x + ray.x * progress,
                y: origin.y + ray.y * progress,
                z: origin.z + ray.z * progress
            },
            normal: self.normal_applied,
            object: Some(self),
            light: None,
        })
    }
    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: {position.x % 1.0},
            y: {position.y % 1.0},
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

        if (intersection_point - center).len() > self.radius_applied {
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

        if (intersection_point - center).len() > self.radius_applied {
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
    fn apply_transform(&mut self) {
        self.axis = Vector{
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        self.axis.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        self.base = self.transform.pos - self.axis * (self.height / 2.0);
        self.top = self.transform.pos + self.axis * (self.height / 2.0);
        self.radius_applied = self.radius * self.transform.scale;
        self.height_applied = self.height * self.transform.scale;
    }

    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let distance = origin - self.base; // W

        let a = 1.0 /*ray.dot_product(ray) car normalisé */ - (ray.dot_product(self.axis)).powi(2);
        let b = 2.0 * (ray.dot_product(distance) - ray.dot_product(self.axis) * distance.dot_product(self.axis));
        let c = distance.dot_product(distance) - distance.dot_product(self.axis).powi(2) - self.radius_applied.powi(2);

        let result = resolve_quadratic_equation(a, b, c);

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|fst, snd| fst.partial_cmp(snd).unwrap());
        if smallest_result == None { return None; }

        let intersection_point = origin + ray * *smallest_result.unwrap();

        if -self.height_applied / 2.0 <= (intersection_point - self.transform.pos).dot_product(self.axis) && (intersection_point - self.transform.pos).dot_product(self.axis) <= self.height_applied / 2.0 { // too far from center
            let normal = intersection_point - (self.base + self.axis * (intersection_point - self.base).dot_product(self.axis)); // Cos(teta) = A/H

            return Some ( Intersection {
                intersection_point,
                normal,
                object: Some(self),
                light: None,
            })
        }
        if (intersection_point - self.base).dot_product(self.axis) < 0.0 {
            return self.base_intersection(ray, origin, self.axis * -1.0, self.base);
        }
        if (intersection_point - self.base).dot_product(self.axis) > self.height_applied {
            return self.base_intersection(ray, origin,  self.axis, self.top);
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
    fn apply_transform(&mut self) {
        self.axis = Vector{
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        self.axis.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        self.base = self.transform.pos - self.axis * (self.height / 2.0);
        self.top = self.transform.pos + self.axis * (self.height / 2.0);
        self.radius_applied = self.radius * self.transform.scale;
        self.height_applied = self.height * self.transform.scale;
    }

    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let distance = origin - self.top; // W

        let radius_constant = self.radius_applied.powi(2) / self.height_applied.powi(2);

        let a = 1.0 - radius_constant * (ray.dot_product(self.axis)).powi(2) - (ray.dot_product(self.axis)).powi(2);
        let b = 2.0 * (ray.dot_product(distance) - radius_constant * ray.dot_product(self.axis) * distance.dot_product(self.axis) - ray.dot_product(self.axis) * distance.dot_product(self.axis));
        let c = distance.dot_product(distance) - radius_constant * distance.dot_product(self.axis).powi(2) - distance.dot_product(self.axis).powi(2);

        let result = resolve_quadratic_equation(a, b, c);

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|fst, snd| fst.partial_cmp(snd).unwrap());
        if smallest_result == None { return None; }

        let intersection_point = origin + ray * *smallest_result.unwrap();
        if 0.0 <= (intersection_point - self.base).dot_product(self.axis) && (intersection_point - self.base).dot_product(self.axis) <= self.height_applied { // too far from center*/
            let cos_angle = self.axis.dot_product(self.top - intersection_point);
            let normal = (intersection_point - (self.top - self.axis * ((self.top - intersection_point).len2() / cos_angle))).normalize();

            return Some ( Intersection {
                intersection_point,
                normal,
                object: Some(self),
                light: None,
            });
        }
        if (intersection_point - self.top).dot_product(self.axis) < 0.0 {
            return self.base_intersection(ray, origin, self.axis * -1.0, self.base);
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
    fn apply_transform(&mut self) {
        self.point_a_applied = self.point_a.clone();
        self.point_a_applied.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        self.point_a_applied = self.point_a_applied + self.transform.pos;
        self.point_b_applied = self.point_b.clone();
        self.point_b_applied.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        self.point_b_applied = self.point_b_applied + self.transform.pos;
        self.point_c_applied = self.point_c.clone();
        self.point_c_applied.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        self.point_c_applied = self.point_c_applied + self.transform.pos;
        self.normal = (self.point_b_applied - self.point_a_applied).cross_product(self.point_c_applied - self.point_a_applied).normalize();
    }

    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let denom = ray.normalize().dot_product(self.normal);
        if denom == 0.0 {
            return None
        }
        let progress = (((self.point_a_applied + self.point_b_applied + self.point_c_applied) / 3.0) - origin).dot_product(self.normal) / denom;
        if progress < 0.0 {
            return None
        }
        let intersection_point = Vector{
            x: origin.x + ray.x * progress,
            y: origin.y + ray.y * progress,
            z: origin.z + ray.z * progress
        };

        let cross = (self.point_b_applied - self.point_a_applied).cross_product(intersection_point - self.point_a_applied);
        if self.normal.dot_product(cross) < 0.0 {
            return None;
        }

        let cross = (self.point_c_applied - self.point_b_applied).cross_product(intersection_point - self.point_b_applied);
        if self.normal.dot_product(cross) < 0.0 {
            return None;
        }

        let cross = (self.point_a_applied - self.point_c_applied).cross_product(intersection_point - self.point_c_applied);
        if self.normal.dot_product(cross) < 0.0 {
            return None;
        }

        Some ( Intersection {
            intersection_point,
            normal: if self.normal.dot_product(origin - intersection_point) < 0.0 { self.normal * -1.0 } else { self.normal },
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
