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

pub struct Renderer {
    pub camera: Camera,
    pub primitives: Vec::<Box::<dyn Object>>,
    pub lights: Lights,
}

impl Renderer {

    pub fn new() -> Renderer {
        let renderer = Renderer {
            camera: Camera::default(),
            primitives: Vec::new(),
            lights: Lights {
                lights: Vec::new(),
                ambiant: Vec::new(),
            },
        };
        renderer
    }

    pub fn render(&mut self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {
                let camera_to_pixel = self.camera.get_pixel_vector(j, i);
                    let intersect = self.primitives[0].intersection(camera_to_pixel, self.camera.transform.pos);
                    if intersect != None {
                        let light_vector = (self.lights.lights[0].get_transform().pos - intersect.unwrap().end).normalize();
                        let normal_vector = (intersect.unwrap().end - intersect.unwrap().origin).normalize();

                        let ambient = self.camera.ambient * self.primitives[0].get_texture().ambient;
                        let diffuse = light_vector.dot_product(normal_vector).max(0.0) * self.camera.diffuse * self.primitives[0].get_texture().diffuse;

                        let reflected = light_vector.reflect(normal_vector).normalize();
                        let view = (camera_to_pixel * -1.0).normalize();
                        let specular = self.camera.specular * self.primitives[0].get_texture().specular * reflected.dot_product(view).max(0.0).powf(self.primitives[0].get_texture().shininess);

                        pixels.extend(&[
                            ((ambient + diffuse) * self.primitives[0].get_texture().color.r as f64 + specular * 255.0).clamp(0.0, 255.0) as u8,
                            ((ambient + diffuse) * self.primitives[0].get_texture().color.g as f64 + specular * 255.0).clamp(0.0, 255.0) as u8,
                            ((ambient + diffuse) * self.primitives[0].get_texture().color.b as f64 + specular * 255.0).clamp(0.0, 255.0) as u8
                        ]);
                    } else {
                        pixels.extend(&[0x00, 0x00, 0x00]);
                    }
                }
            }
        pixels
    }

    pub fn get_renderer_from_file(file: String) -> Renderer {
        let data = fs::read_to_string(file).expect("Unable to read file");
        let json: Value = serde_json::from_str(&data.to_string()).unwrap();
        let parser = Parser{};
        parser.get_renderer_from_json(&json)
    }

}
