//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// lights
//

use super::camera::Camera;
use super::primitives::{Intersection, Object};
use super::types::{Color, Transform};
use crate::vector::{resolve_quadratic_equation, Vector};
use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Point {
    pub transform: Transform,
    pub color: Color,
    pub strength: f64,
    pub radius: f64,
    pub falloff: i32,
    pub visible: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Directional {
    pub transform: Transform,
    pub color: Color,
    pub strength: f64,
    pub visible: bool,
}

pub trait Light: erased_serde::Serialize {
    fn move_obj(&mut self, offset: Transform);
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
    fn get_color(&self) -> Color;
    fn get_visible(&self) -> bool;
    fn set_color(&mut self, new: Color);
    fn get_strength(&self) -> f64;
    fn set_strength(&mut self, new: f64);
    fn get_radius(&self) -> f64;
    fn set_radius(&mut self, new: f64);
    fn get_falloff(&self) -> i32;
    fn set_falloff(&mut self, new: i32);
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
    fn light_is_intersected(
        &self,
        light_vector: Vector,
        intersect: &Intersection,
        normal_vector: Vector,
        camera: Camera,
        primitives: &[Box<dyn Object + Send + Sync>],
    ) -> bool;
    fn calculate_light(
        &self,
        intersect: &Intersection,
        camera_to_pixel: Vector,
        camera: Camera,
        primitives: &[Box<dyn Object + Send + Sync>],
    ) -> Vector;
}

impl Light for Point {
    fn move_obj(&mut self, offset: Transform) {
        self.transform = self.transform + offset
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, new: Transform) {
        self.transform = new
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_visible(&self) -> bool {
        self.visible
    }

    fn set_color(&mut self, new: Color) {
        self.color = new
    }

    fn get_strength(&self) -> f64 {
        self.strength
    }

    fn set_strength(&mut self, new: f64) {
        self.strength = new
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }

    fn set_radius(&mut self, new: f64) {
        self.radius = new
    }

    fn get_falloff(&self) -> i32 {
        self.falloff
    }

    fn set_falloff(&mut self, new: i32) {
        self.falloff = new
    }

    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(
            ray.dot_product(ray), // could be 1 if normalized
            2.0 * (ray.dot_product(diff)),
            (diff.dot_product(diff)) - self.radius.powi(2),
        );

        let smallest_result: Option<&f64> = result
            .iter()
            .filter(|number| **number > 0.0)
            .min_by(|a, b| a.partial_cmp(b).unwrap());

        if let Some(smallest) = smallest_result {
            let point = Vector {
                x: origin.x + ray.x * smallest,
                y: origin.y + ray.y * smallest,
                z: origin.z + ray.z * smallest,
            };
            Some(Intersection {
                normal: point - self.transform.pos,
                intersection_point: point,
                object: None,
                light: Some(self),
            })
        } else {
            None
        }
    }

    fn light_is_intersected(
        &self,
        light_vector: Vector,
        intersect: &Intersection,
        normal_vector: Vector,
        camera: Camera,
        primitives: &[Box<dyn Object + Send + Sync>],
    ) -> bool {
        for object_current in primitives.iter() {
            match object_current.intersection(
                light_vector,
                intersect.intersection_point + (normal_vector * camera.shadow_bias),
            ) {
                None => continue,
                Some(shadow_intersect) => {
                    if (shadow_intersect.intersection_point - intersect.intersection_point).len()
                        < (self.transform.pos - intersect.intersection_point).len()
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn calculate_light(
        &self,
        intersect: &Intersection,
        camera_to_pixel: Vector,
        camera: Camera,
        primitives: &[Box<dyn Object + Send + Sync>],
    ) -> Vector {
        let normal_vector = intersect.normal.normalize();
        let light_vector = (self.get_transform().pos - intersect.intersection_point).normalize();
        let mut light_uncovered = 1.0;

        if !camera.smooth_shadow {
            if self.light_is_intersected(light_vector, intersect, normal_vector, camera, primitives)
            {
                return Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
            }
        } else {
            let mut light_reached: i16 = 0;
            for _ in 0..camera.smooth_shadow_step {
                let inter_to_light = self.get_transform().pos
                    + Vector::get_random_point_in_sphere(self.get_radius())
                    - intersect.intersection_point;
                if !self.light_is_intersected(
                    inter_to_light.normalize(),
                    intersect,
                    normal_vector,
                    camera,
                    primitives,
                ) {
                    light_reached += 1;
                }
            }
            light_uncovered = light_reached as f64 / camera.smooth_shadow_step as f64;
        }
        let diffuse = light_vector.dot_product(normal_vector).max(0.0)
            * camera.diffuse
            * intersect.object.unwrap().get_texture().diffuse;

        let reflected = light_vector.reflect(normal_vector).normalize();
        let view = (camera_to_pixel * -1.0).normalize();
        let specular = camera.specular
            * intersect.object.unwrap().get_texture().specular
            * reflected
                .dot_product(view)
                .max(0.0)
                .powf(intersect.object.unwrap().get_texture().shininess);
        let distance = intersect
            .intersection_point
            .distance(self.get_transform().pos);
        let light_falloff = (self.get_strength() / distance.powi(self.get_falloff())).max(0.0);
        let texture_coordinates = intersect.object.unwrap().surface_position(
            intersect.intersection_point - intersect.object.unwrap().get_transform().pos,
        );
        intersect
            .object
            .unwrap()
            .get_texture()
            .texture(texture_coordinates.x, texture_coordinates.y)
            .as_vector()
            * self.get_color().as_vector()
            * diffuse
            * light_falloff
            * light_uncovered
            + self.get_color().as_vector() * specular * light_falloff * light_uncovered
    }
}

impl Light for Directional {
    fn move_obj(&mut self, offset: Transform) {
        self.transform = self.transform + offset
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, new: Transform) {
        self.transform = new
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_visible(&self) -> bool {
        self.visible
    }

    fn set_color(&mut self, new: Color) {
        self.color = new
    }

    fn get_strength(&self) -> f64 {
        self.strength
    }

    fn set_strength(&mut self, new: f64) {
        self.strength = new
    }

    fn get_radius(&self) -> f64 {
        1.0
    }

    fn set_radius(&mut self, _new: f64) {}
    fn get_falloff(&self) -> i32 {
        0
    }

    fn set_falloff(&mut self, _new: i32) {}
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(
            ray.dot_product(ray), // could be 1 if normalized
            2.0 * (ray.dot_product(diff)),
            diff.dot_product(diff),
        );

        let smallest_result: Option<&f64> = result
            .iter()
            .filter(|number| **number > 0.0)
            .min_by(|a, b| a.partial_cmp(b).unwrap());

        if let Some(smallest) = smallest_result {
            let point = Vector {
                x: origin.x + ray.x * smallest,
                y: origin.y + ray.y * smallest,
                z: origin.z + ray.z * smallest,
            };
            return Some(Intersection {
                normal: point - self.transform.pos,
                intersection_point: point,
                object: None,
                light: Some(self),
            });
        }
        None
    }

    fn light_is_intersected(
        &self,
        light_vector: Vector,
        intersect: &Intersection,
        normal_vector: Vector,
        camera: Camera,
        primitives: &[Box<dyn Object + Send + Sync>],
    ) -> bool {
        for object_current in primitives.iter() {
            match object_current.intersection(
                light_vector,
                intersect.intersection_point + (normal_vector * camera.shadow_bias),
            ) {
                None => continue,
                Some(shadow_intersect) => {
                    if (shadow_intersect.intersection_point - intersect.intersection_point).len()
                        < (self.transform.pos - intersect.intersection_point).len()
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn calculate_light(
        &self,
        intersect: &Intersection,
        camera_to_pixel: Vector,
        camera: Camera,
        primitives: &[Box<dyn Object + Send + Sync>],
    ) -> Vector {
        let normal_vector = intersect.normal.normalize();
        let mut light_uncovered = 1.0;

        if !camera.smooth_shadow {
            if self.light_is_intersected(
                self.transform.pos,
                intersect,
                normal_vector,
                camera,
                primitives,
            ) {
                return Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
            }
        } else {
            let mut light_reached: i16 = 0;
            for _ in 0..camera.smooth_shadow_step {
                let inter_to_light = self.transform.pos + Vector::get_random_point_in_sphere(0.0)
                    - intersect.intersection_point;
                if !self.light_is_intersected(
                    inter_to_light.normalize(),
                    intersect,
                    normal_vector,
                    camera,
                    primitives,
                ) {
                    light_reached += 1;
                }
            }
            light_uncovered = light_reached as f64 / camera.smooth_shadow_step as f64;
        }
        let diffuse = self.transform.pos.dot_product(normal_vector).max(0.0)
            * camera.diffuse
            * intersect.object.unwrap().get_texture().diffuse;

        let reflected = self.transform.pos.reflect(normal_vector).normalize();
        let view = (camera_to_pixel * -1.0).normalize();
        let specular = camera.specular
            * intersect.object.unwrap().get_texture().specular
            * reflected
                .dot_product(view)
                .max(0.0)
                .powf(intersect.object.unwrap().get_texture().shininess);
        let texture_coordinates = intersect.object.unwrap().surface_position(
            intersect.intersection_point - intersect.object.unwrap().get_transform().pos,
        );
        intersect
            .object
            .unwrap()
            .get_texture()
            .texture(texture_coordinates.x, texture_coordinates.y)
            .as_vector()
            * self.get_color().as_vector()
            * diffuse
            * light_uncovered
            + self.get_color().as_vector() * specular * light_uncovered
    }
}

serialize_trait_object!(Light);

#[derive(Deserialize, Serialize)]
pub struct Ambient {
    pub color: Color,
    pub strength: f64,
}

impl Ambient {
    pub fn default() -> Ambient {
        Ambient {
            color: Color::default(),
            strength: 80.0,
        }
    }
}

#[derive(Serialize)]
pub struct Lights {
    pub lights: Vec<Box<dyn Light + Send + Sync>>,
    pub ambient: Vec<Ambient>,
}

impl Lights {
    pub fn default() -> Lights {
        let mut lights = Lights {
            lights: Vec::new(),
            ambient: Vec::new(),
        };
        lights.ambient.push(Ambient::default());
        lights
    }
}
