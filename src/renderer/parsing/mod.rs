//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// json parsing
//

use crate::vectors;
use serde_json::Value;
use vectors::Vector;
use super::camera::{Lens, Camera};
use super::primitives::{Sphere, Plane, Cylinder, Cone, Object};
use super::lights::{Point, Ambient, Light, Lights};
use super::renderer_common::{Transform, Color, Texture};

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

    pub fn get_transform_from_json(&self, json: &Value) -> Transform {
        Transform {
            pos: if json["pos"].is_object() {self.get_vector_from_json(&json["pos"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            rotation: if json["rotation"].is_object() {self.get_vector_from_json(&json["rotation"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            scale: if json["scale"].is_object() {self.get_vector_from_json(&json["scale"])} else {Vector {x: 1.0, y: 1.0, z: 1.0}},
        }
    }

    pub fn get_lens_from_json(&self, json: &Value, height: i64, width: i64) -> Lens {
        Lens {
            height: height,
            width: width,
            distance: json["distance"].as_f64().unwrap_or(0.0),
            vector_to_first_pixel: if json["vector_to_first_pixel"].is_object() {self.get_vector_from_json(&json["vector_to_first_pixel"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
        }
    }

    pub fn get_camera_from_json(&self, json: &Value, height: i64, width: i64) -> Camera {
        let mut camera = Camera {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            lens: if json["lens"].is_object() {self.get_lens_from_json(&json["lens"], height, width)} else {Lens::default()},
            fov: json["fov"].as_i64().unwrap_or(60),
            smooth_shadow: json["smooth_shadow"].as_bool().unwrap_or(true),
            smooth_shadow_step: json["smooth_shadow_step"].as_i64().unwrap_or(50) as i16,
            diffuse: json["diffuse"].as_f64().unwrap_or(0.7),
            ambient: json["ambient"].as_f64().unwrap_or(0.3),
            specular: json["specular"].as_f64().unwrap_or(0.6),
            shadow_bias: json["shadow_bias"].as_f64().unwrap_or(1e-14),
        };
        camera.calculate_lens_distance();
        let vector_director = Vector {x: 0.0, y: camera.lens.distance, z: 0.0};
        camera.lens.vector_to_first_pixel = Vector {x: camera.transform.pos.x, y: camera.transform.pos.y, z: camera.transform.pos.z};
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + Vector {x:0.0, y:0.0, z:1.0} * (camera.lens.height as f64 / 2.0);
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + vector_director;
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + Vector {x: -1.0, y: 0.0, z: 0.0} * (camera.lens.width as f64 / 2.0);
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
            shininess: json["shininess"].as_f64().unwrap_or(4.0),
        }
    }

    pub fn get_sphere_from_json(&self, json: &Value) -> Box::<Sphere> {
        Box::new(
            Sphere {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                radius: json["radius"].as_f64().unwrap_or(1.0),
            }
        )
    }

    pub fn get_plane_from_json(&self, json: &Value) -> Box::<Plane> {
        Box::new(
            Plane {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                normal: if json["normal"].is_object(){self.get_vector_from_json(&json["normal"])} else {Vector {x: 0.0, y: 0.0, z: 1.0}},
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
            }
        )
    }

    pub fn get_objects_from_json(&self, json: &Value) -> Vec::<Box::<dyn Object>> {
        let mut objects: Vec::<Box::<dyn Object>> = Vec::new();

        if json["spheres"].is_array() {
            for sphere in json["spheres"].as_array().unwrap().iter() {
                objects.push(self.get_sphere_from_json(sphere))
            }
        }
        if json["planes"].is_array() {
            for plane in json["planes"].as_array().unwrap().iter() {
                objects.push(self.get_plane_from_json(plane))
            }
        }
        if json["cylinders"].is_array() {
            for cylinder in json["cylinders"].as_array().unwrap().iter() {
                objects.push(self.get_cylinder_from_json(cylinder))
            }
        }
        if json["cones"].is_array() {
            for cone in json["cones"].as_array().unwrap().iter() {
                objects.push(self.get_cone_from_json(cone))
            }
        }
        objects
    }

    pub fn get_point_from_json(&self, json: &Value) -> Box::<Point> {
        Box::new(
            Point {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                color: if json["color"].is_object() {self.get_color_from_json(&json["color"])} else {Color::default()},
                strength: json["strength"].as_f64().unwrap_or(80.0),
                radius: json["radius"].as_f64().unwrap_or(1.0),
                falloff: json["falloff"].as_i64().unwrap_or(2) as i32,
            }
        )
    }

    pub fn get_object_lights_from_json(&self, json: &Value) -> Vec::<Box::<dyn Light>> {
        let mut lights: Vec::<Box::<dyn Light>> = Vec::new();

        if json["point"].is_array(){
            for point in json["point"].as_array().unwrap().iter() {
                lights.push(self.get_point_from_json(point))
            }
        }
        lights
    }

    pub fn get_ambient_from_json(&self, json: &Value) -> Ambient {
        let color_val: Color;

        if json["color"].is_object() {color_val = self.get_color_from_json(&json["transform"]);}
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
        let objects: Vec::<Box::<dyn Light>>;

        if json["objects"].is_object() { objects = self.get_object_lights_from_json(&json["objects"]);}
        else { objects = Vec::new()}
        Lights {
            lights: objects,
            ambient: self.get_ambients_from_json(&json),
        }
    }

}
