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
use std::process::exit;
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

    fn calculate_light(&self, light: &Box<dyn Light>, intersect: &Intersection, camera_to_pixel: Vector) -> Vector {
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

        let reflected = light_vector.reflect(normal_vector).normalize();
        let view = (camera_to_pixel * -1.0).normalize();
        let specular = self.camera.specular * intersect.object.get_texture().specular * reflected.dot_product(view).max(0.0).powf(intersect.object.get_texture().shininess);
        let distance = intersect.intersection_point.distance(light.get_transform().pos);
        let light_falloff = (light.get_strength() / distance.powi(light.get_falloff())).max(0.0);
        intersect.object.get_texture().color.as_vector() * light.get_color().as_vector() * diffuse * light_falloff * light_uncovered + light.get_color().as_vector() * specular * light_falloff * light_uncovered
    }

    fn found_nearest_intersection(&self, camera_to_pixel: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

         for object in self.primitives.iter() {
            let intersect = object.intersection(camera_to_pixel, self.camera.transform.pos);

            if intersect.is_some() {
                let inters = intersect.unwrap();
                let distance_found = (inters.intersection_point - self.camera.transform.pos).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }
        found_intersection
    }

    pub fn render(&self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {
                let camera_to_pixel = self.camera.get_pixel_vector(j, i);
                let maybe_intersect = self.found_nearest_intersection(camera_to_pixel);
                if let Some(intersect) = maybe_intersect {
                    let mut color = intersect.object.get_texture().color.as_vector() * self.camera.ambient * intersect.object.get_texture().ambient;
                    for light in self.lights.lights.iter() {
                        color = color + self.calculate_light(light, &intersect, camera_to_pixel);
                    }
                    pixels.extend(&[
                        ((color.x).clamp(0.0, 1.0) * 255.0) as u8,
                        ((color.y).clamp(0.0, 1.0) * 255.0) as u8,
                        ((color.z).clamp(0.0, 1.0) * 255.0) as u8
                    ]);
                } else {
                    let color_a = Vector {x: 0.0, y: 212.0, z: 255.0} * (1.0/255.0);
                    let color_b = Vector {x: 2.0, y: 0.0, z: 36.0} * (1.0/255.0);
                    let percent = i as f64 / self.camera.lens.height as f64;
                    let result = color_a + (color_b - color_a) * percent as f64;
                    pixels.extend(&[
                        (result.x * 255.0 as f64) as u8,
                        (result.y * 255.0 as f64) as u8,
                        (result.z * 255.0 as f64) as u8
                    ]);
                }
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
