//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// json parsing
//

use crate::vectors;
use serde_json::Value;
use vectors::Vector;
use super::Renderer;
use super::camera::{Lens, Camera};
use super::primitives::{Sphere, Plane, Cylinder, Cone, Object};
use super::lights::{Point, Ambient, Light, Lights};
use super::renderer_common::{Transform, Color, Texture};
use std::fs;

pub struct Parser {
}

impl Parser {
    pub fn get_vector_from_json(&self, json: &Value) -> Vector {
        Vector {
            x: json["x"].as_f64().unwrap_or(0.0),
            y: json["y"].as_f64().unwrap_or(0.0),
            z: json["z"].as_f64().unwrap_or(0.0),
        }
    }

    fn get_transform_from_json(&self, json: &Value) -> Transform {
        Transform {
            pos: if json["pos"].is_object() {self.get_vector_from_json(&json["pos"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            rotation: if json["rotation"].is_object() {self.get_vector_from_json(&json["rotation"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            scale: if json["scale"].is_object() {self.get_vector_from_json(&json["scale"])} else {Vector {x: 1.0, y: 1.0, z: 1.0}},
        }
    }

    pub fn get_camera_from_json(&self, json: &Value, height: i64, width: i64) -> Camera {
        let mut camera = Camera {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            lens: Lens::default(height, width),
            fov: json["fov"].as_i64().unwrap_or(60),
            smooth_shadow: json["smooth_shadow"].as_bool().unwrap_or(true),
            smooth_shadow_step: json["smooth_shadow_step"].as_i64().unwrap_or(50) as i16,
            diffuse: json["diffuse"].as_f64().unwrap_or(0.7),
            ambient: json["ambient"].as_f64().unwrap_or(0.3),
            specular: json["specular"].as_f64().unwrap_or(0.6),
            shadow_bias: json["shadow_bias"].as_f64().unwrap_or(1e-14),
            aces_tone_mapping: json["aces_tone_mapping"].as_bool().unwrap_or(true),
            recursivity: json["recursivity"].as_i64().unwrap_or(5),
            reflection_samples: json["reflection_samples"].as_i64().unwrap_or(5),
            threads: json["threads"].as_u64().unwrap_or(8),
            progression: json["progression"].as_bool().unwrap_or(false),
            super_sampling: json["super_sampling"].as_u64().unwrap_or(1),
            super_sampling_precision: json["super_sampling_precision"].as_u64().unwrap_or(10),
            image_buffer_size: json["image_buffer_size"].as_u64().unwrap_or(1),
        };
        camera.calculate_lens_distance();
        let vector_director = Vector {x: 0.0, y: camera.lens.distance, z: 0.0};
        camera.lens.vector_to_first_pixel = Vector {x: camera.transform.pos.x, y: camera.transform.pos.y, z: camera.transform.pos.z};
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + Vector {x:0.0, y:0.0, z:1.0} * (camera.lens.height as f64 / 2.0);
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + vector_director;
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + Vector {x: -1.0, y: 0.0, z: 0.0} * (camera.lens.width as f64 / 2.0);

        if camera.threads < 1 {
            camera.threads = 1;
        }

        camera
    }

    pub fn get_color_from_json(&self, json: &Value) -> Color {
        Color {
            r: (json["r"].as_f64().unwrap_or(255.0) % 256.0),
            g: (json["g"].as_f64().unwrap_or(255.0) % 256.0),
            b: (json["b"].as_f64().unwrap_or(255.0) % 256.0),
        }
    }

    pub fn get_texture_from_json(&self, json: &Value) -> Texture {
        Texture {
            texture_type: json["texture_type"].as_u64().unwrap_or(1),
            color: if json["color"].is_object(){self.get_color_from_json(&json["color"])} else {Color::default()},
            diffuse: json["diffuse"].as_f64().unwrap_or(0.7),
            ambient: json["ambient"].as_f64().unwrap_or(0.1),
            specular: json["specular"].as_f64().unwrap_or(0.4),
            metalness: json["metalness"].as_f64().unwrap_or(0.1),
            shininess: json["shininess"].as_f64().unwrap_or(4.0),
            roughness: json["roughness"].as_f64().unwrap_or(0.25),
        }
    }

    pub fn get_sphere_from_json(&self, json: &Value) -> Box::<Sphere> {
        Box::new(
            Sphere {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                radius: json["radius"].as_f64().unwrap_or(1.0),
                children: if json["children"].is_object() {self.get_objects_from_json(&json["children"])} else {Vec::new()},
            }
        )
    }

    pub fn get_plane_from_json(&self, json: &Value) -> Box::<Plane> {
        Box::new(
            Plane {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                normal: if json["normal"].is_object(){self.get_vector_from_json(&json["normal"])} else {Vector {x: 0.0, y: 0.0, z: 1.0}},
                children: if json["children"].is_object() {self.get_objects_from_json(&json["children"])} else {Vec::new()},
            }
        )
    }

    pub fn get_cylinder_from_json(&self, json: &Value) -> Box::<Cylinder> {
        Box::new(
            Cylinder {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                height: json["height"].as_f64().unwrap_or(2.0),
                radius: json["radius"].as_f64().unwrap_or(1.0),
                children: if json["children"].is_object() {self.get_objects_from_json(&json["children"])} else {Vec::new()},
            }
        )
    }

    pub fn get_cone_from_json(&self, json: &Value) -> Box::<Cone> {
        Box::new(
            Cone {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                height: json["height"].as_f64().unwrap_or(3.0),
                radius: json["radius"].as_f64().unwrap_or(1.0),
                children: if json["children"].is_object() {self.get_objects_from_json(&json["children"])} else {Vec::new()},
            }
        )
    }

    pub fn get_object_from_json(&self, json: &Value) -> Option<Box::<dyn Object + Send + Sync>> {
        if json["type"].is_string() {
            return match json["type"].as_str().unwrap() {
                "sphere" => Some(self.get_sphere_from_json(json)),
                "plane" => Some(self.get_plane_from_json(json)),
                "cylinder" => Some(self.get_cylinder_from_json(json)),
                "cone" => Some(self.get_cone_from_json(json)),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn get_objects_from_json(&self, json: &Value) -> Vec::<Box::<dyn Object + Send + Sync>> {
        let mut objects: Vec::<Box::<dyn Object + Send + Sync>> = Vec::new();

        if json.is_array() {
            for object in json.as_array().unwrap().iter() {
                if let Some(result) = self.get_object_from_json(object) {
                objects.push(result)
                }
            }
        }
        objects
    }

    pub fn get_point_from_json(&self, json: &Value) -> Box::<Point> {
        Box::new(
            Point {
                visible: json["visible"].as_bool().unwrap_or(false),
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                color: if json["color"].is_object() {self.get_color_from_json(&json["color"])} else {Color::default()},
                strength: json["strength"].as_f64().unwrap_or(80.0),
                radius: json["radius"].as_f64().unwrap_or(1.0),
                falloff: json["falloff"].as_i64().unwrap_or(2) as i32,
            }
        )
    }

    pub fn get_object_lights_from_json(&self, json: &Value) -> Vec::<Box::<dyn Light + Send + Sync>> {
        let mut lights: Vec::<Box::<dyn Light + Send + Sync>> = Vec::new();

        if json["point"].is_array(){
            for point in json["point"].as_array().unwrap().iter() {
                lights.push(self.get_point_from_json(point))
            }
        }
        lights
    }

    pub fn get_ambient_from_json(&self, json: &Value) -> Ambient {
        let color_val: Color;

        if json["color"].is_object() {color_val = self.get_color_from_json(&json["color"]);}
        else {color_val = Color::default();}
        Ambient {
            color: color_val,
            strength: json["strength"].as_f64().unwrap_or(80.0),
        }
    }

    pub fn get_ambients_from_json(&self, json: &Value) -> Vec::<Ambient> {
        let mut lights: Vec::<Ambient> = Vec::new();

        if json["ambient"].is_array() {
            for ambient in json["ambient"].as_array().unwrap().iter() {
                lights.push(self.get_ambient_from_json(ambient))
            }
        }
        lights
    }

    pub fn get_lights_from_json(&self, json: &Value) -> Lights {
        let objects: Vec::<Box::<dyn Light + Send + Sync>>;

        if json["objects"].is_object() { objects = self.get_object_lights_from_json(&json["objects"]);}
        else { objects = Vec::new()}
        Lights {
            lights: objects,
            ambient: self.get_ambients_from_json(&json),
        }
    }

    fn move_primitives(&self, primitives: &mut Vec::<Box::<dyn Object + Send + Sync>>, offset: Transform) {
        for element in primitives {
            element.move_obj(offset);
        }
    }

    fn move_lights(&self, lights: &mut Vec::<Box::<dyn Light + Send + Sync>>, offset: Transform) {
        for element in lights {
            element.move_obj(offset);
        }
    }

    fn get_scene_from_json(&self, renderer: &mut Renderer, json: &Value, offset: Transform) {
        if json["primitives"].is_object() {
            let mut primitives = self.get_objects_from_json(&json["primitives"]);
            self.move_primitives(&mut primitives, offset);
            renderer.primitives.extend(primitives);
        }
        if json["lights"]["objects"].is_object() {
            let mut lights = self.get_object_lights_from_json(&json["lights"]["objects"]);
            self.move_lights(&mut lights, offset);
            renderer.lights.lights.extend(lights);
        }
        renderer.lights.ambient.extend(if json["lights"]["ambiant"].is_object() {self.get_ambients_from_json(&json["lights"]["ambiant"])} else {Vec::new()});
    }

    fn get_scenes_from_json(&self, renderer: &mut Renderer, json: &Value, path_taken: &mut Vec<String>) {
        if json["scenes"].is_array() {
            for scene in json["scenes"].as_array().unwrap().iter() {
                let filename = scene["file"].as_str().unwrap().to_string();
                if scene["file"].is_string() && self.get_json(&filename).is_some() {
                    let scene_json = self.get_json(&filename).unwrap();
                    self.get_scene_from_json(renderer, &scene_json, if scene["transform"].is_object() {self.get_transform_from_json(&scene["transform"])} else {Transform::default()});
                    if path_taken.contains(&filename) == false {
                        path_taken.push(filename);
                        self.get_scenes_from_json(renderer, &scene_json, path_taken);
                        path_taken.pop();
                    } else {
                        print!("inclusion of scene {} impossible\nbecause of configuration, it contains itself and will create an infinite loop if included\n the problem detected in the {} config file\n", filename, path_taken.last().unwrap_or(&"root".to_string()))
                    }
                }
            }
        }
    }

    pub fn get_renderer_from_json(&self, json: &Value, height: i64, width: i64) -> Renderer {
        let mut renderer: Renderer = Renderer {
            camera: if json["camera"].is_object() {self.get_camera_from_json(&json["camera"], height, width)} else {Camera::default(1920, 1080)},
            primitives: if json["primitives"].is_array() {self.get_objects_from_json(&json["primitives"])} else {Vec::new()},
            lights: if json["lights"].is_object() {self.get_lights_from_json(&json["lights"])} else {Lights::default()},
        };
        self.get_scenes_from_json(&mut renderer, json, &mut Vec::new());
        renderer
    }

    pub fn get_json(&self, file: &String) -> Option<Value> {
        let data = fs::read_to_string(file).expect("Unable to read file");
        serde_json::from_str(&data.to_string()).unwrap_or(None)
    }

}
