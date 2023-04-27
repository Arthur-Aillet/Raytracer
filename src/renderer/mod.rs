//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer common structures
//

mod camera;
mod primitives;
mod lights;
mod parsing;
mod renderer_common;

use std::fs;
use serde_json::Value;
use camera::{Camera};
use primitives::{Object};
use lights::Lights;
use parsing::Parser;
use crate::renderer::lights::Light;
use crate::renderer::primitives::Intersection;
use crate::vectors::Vector;

pub struct Renderer {
    pub camera: Camera,
    pub primitives: Vec<Box<dyn Object>>,
    pub lights: Lights,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            camera: Camera::default(),
            primitives: Vec::new(),
            lights: Lights {
                lights: Vec::new(),
                ambient: Vec::new(),
            },
        }
    }

    fn light_is_intersected(&self, light_vector: Vector, intersect: &Intersection, light: &Box<dyn Light>, normal_vector: Vector) -> bool {
        for object_current in self.primitives.iter() {
            match object_current.intersection(light_vector, intersect.intersection_point + (normal_vector * self.camera.shadow_bias)) {
                None => { continue }
                Some(shadow_intersect) => {
                    if (shadow_intersect.intersection_point - intersect.intersection_point).len() < (light.get_transform().pos - intersect.intersection_point).len() {
                        return true
                    }
                }
            }
        }
        false
    }

    fn calculate_light(&self, light: &Box<dyn Light>, intersect: &Intersection, ray: Vector) -> Vector {
        let normal_vector = intersect.normal.normalize();
        let light_vector = (light.get_transform().pos - intersect.intersection_point).normalize();
        let mut light_uncovered = 1.0;

        if self.camera.smooth_shadow == false {
            if self.light_is_intersected(light_vector, intersect, light, normal_vector) {
                return Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }
        } else {
            let mut light_reached: i16 = 0;
            for _ in 0..self.camera.smooth_shadow_step {
                let inter_to_light = light.get_transform().pos + Vector::get_random_point_in_sphere(light.get_radius()) - intersect.intersection_point;
                if self.light_is_intersected(inter_to_light.normalize(), intersect, light, normal_vector) == false {
                    light_reached += 1;
                }
            }
            light_uncovered = light_reached as f64 / self.camera.smooth_shadow_step as f64;
        }
        let diffuse = light_vector.dot_product(normal_vector).max(0.0) * self.camera.diffuse * intersect.object.get_texture().diffuse;
        // GI
        let reflected = light_vector.reflect(normal_vector).normalize();
        let view = (ray * -1.0).normalize();
        let specular = self.camera.specular * intersect.object.get_texture().specular * reflected.dot_product(view).max(0.0).powf(intersect.object.get_texture().shininess);
        let distance = intersect.intersection_point.distance(light.get_transform().pos);
        let light_falloff = (light.get_strength() / distance.powi(light.get_falloff())).max(0.0);
        intersect.object.get_texture().color.as_vector() * light.get_color().as_vector() * diffuse * light_falloff * light_uncovered + light.get_color().as_vector() * specular * light_falloff * light_uncovered
    }

    fn found_nearest_intersection(&self, origin: Vector, camera_to_pixel: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

         for object in self.primitives.iter() {
            let intersect = object.intersection(camera_to_pixel, origin);

            if intersect.is_some() {
                let inters = intersect.unwrap();
                let distance_found = (inters.intersection_point - origin).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }
        found_intersection
    }

    fn get_color_from_ray(&self, origin: Vector, ray: Vector, recursivity: i64) -> Vector {
        if recursivity == 0 {
            return Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
        let maybe_intersect = self.found_nearest_intersection(origin, ray);

        if let Some(intersect) = maybe_intersect {
            let mut self_color = intersect.object.get_texture().color.as_vector() * self.camera.ambient * intersect.object.get_texture().ambient;

            for light in self.lights.lights.iter() {
                self_color = self_color + self.calculate_light(light, &intersect, ray);
            }
            let surface_point = intersect.intersection_point + intersect.normal * self.camera.shadow_bias;

            let reflection_ray = ray.normalize() - intersect.normal.normalize() * 2.0 * intersect.normal.dot_product(ray.normalize());

            self_color = self_color * (1.0 - intersect.object.get_texture().metalness);
            self_color = self_color + self.get_color_from_ray(surface_point, reflection_ray, recursivity - 1) * intersect.object.get_texture().metalness;
            self_color
        } else {
            Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        pixels.reserve((self.camera.lens.width * self.camera.lens.height) as usize);

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {
                let color = self.get_color_from_ray(self.camera.transform.pos, self.camera.get_pixel_vector(j, i), 200);

                pixels.extend(&[
                    ((color.x).clamp(0.0, 1.0) * 255.0) as u8,
                    ((color.y).clamp(0.0, 1.0) * 255.0) as u8,
                    ((color.z).clamp(0.0, 1.0) * 255.0) as u8
                ]);
            }
        }
        pixels
    }

    pub fn get_renderer_from_file(file: String) -> Renderer {
        let data = fs::read_to_string(file).expect("Unable to read file");
        let json: Value = serde_json::from_str(&data.to_string()).unwrap();
        let parser = Parser{};
        Renderer {
            camera: if json["camera"].is_object() {parser.get_camera_from_json(&json["camera"])} else {Camera::default()},
            primitives: if json["primitives"].is_object() {parser.get_objects_from_json(&json["primitives"])} else {Vec::new()},
            lights: if json["lights"].is_object() {parser.get_lights_from_json(&json["lights"])} else {Lights::default()},
        }
    }

}
