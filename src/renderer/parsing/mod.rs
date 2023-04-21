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
use super::lights::{Directional, Ambiant, Light, Lights};
use super::renderer_common::{Transform, Color, Texture};

pub struct Parser {
}

impl Parser {

    pub fn get_vector_from_json(&self, json: &Value) -> Vector {
        let option_x = json["x"].as_f64();
        let option_y = json["y"].as_f64();
        let option_z = json["z"].as_f64();

        let vector = Vector {
            x: option_x.unwrap_or(0.0),
            y: option_y.unwrap_or(0.0),
            z: option_z.unwrap_or(0.0),
        };
        vector
    }

    pub fn get_transform_from_json(&self, json: &Value) -> Transform {
        Transform {
            pos: if json["pos"].is_object() {self.get_vector_from_json(&json["pos"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            rotation: if json["rotation"].is_object() {self.get_vector_from_json(&json["rotation"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            scale: if json["scale"].is_object() {self.get_vector_from_json(&json["scale"])} else {Vector {x: 1.0, y: 1.0, z: 1.0}},
        }
    }

    pub fn get_lens_from_json(&self, json: &Value) -> Lens {
        let option_h = json["height"].as_i64();
        let option_w = json["width"].as_i64();
        let option_d =json["distance"].as_f64();

        Lens {
            height: option_h.unwrap_or(0),
            width: option_w.unwrap_or(0),
            distance: option_d.unwrap_or(0.0),
            vector_to_first_pixel: if json["vector_to_first_pixel"].is_object() {self.get_vector_from_json(&json["vector_to_first_pixel"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
        }
    }

    pub fn get_camera_from_json(&self, json: &Value) -> Camera {
        let option_f = json["fov"].as_i64();
        let option_d = json["diffuse"].as_f64();
        let option_a = json["ambient"].as_f64();
        let option_s = json["specular"].as_f64();

        let mut camera = Camera {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            lens: if json["lens"].is_object() {self.get_lens_from_json(&json["lens"])} else {Lens::default()},
            fov: option_f.unwrap_or(60),
            diffuse: option_d.unwrap_or(0.7),
            ambient: option_a.unwrap_or(0.1),
            specular: option_s.unwrap_or(0.6),
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
        let option_r = json["r"].as_f64();
        let option_g = json["g"].as_f64();
        let option_b = json["b"].as_f64();

        Color {
            r: (option_r.unwrap_or(255.0) % 256.0),
            g: (option_g.unwrap_or(255.0) % 256.0),
            b: (option_b.unwrap_or(255.0) % 256.0),
        }
    }

    pub fn get_texture_from_json(&self, json: &Value) -> Texture {
        let option_t = json["texture_type"].as_u64();
        let option_d = json["diffuse"].as_f64();
        let option_a = json["ambient"].as_f64();
        let option_sp = json["specular"].as_f64();
        let option_sh = json["shininess"].as_f64();

        Texture {
            texture_type: option_t.unwrap_or(1),
            color: if json["color"].is_object(){self.get_color_from_json(&json["color"])} else {Color::default()},
            diffuse: option_d.unwrap_or(0.7),
            ambient: option_a.unwrap_or(0.1),
            specular: option_sp.unwrap_or(0.4),
            shininess: option_sh.unwrap_or(4.0),
        }
    }

    pub fn get_sphere_from_json(&self, json: &Value) -> Box::<Sphere> {
        let option_r = json["radius"].as_f64();

        Box::new(
            Sphere {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                radius: option_r.unwrap_or(1.0),
            }
        )
    }

    pub fn get_plane_from_json(&self, json: &Value) -> Box::<Plane> {
        let option_d = json["vector"].as_f64();

        Box::new(
            Plane {
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                normal: if json["normal"].is_object(){self.get_vector_from_json(&json["normal"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
                distance: option_d.unwrap_or(0.0),
            }
        )
    }

    pub fn get_cylinder_from_json(&self, json: &Value) -> Box::<Cylinder> {
        let option_h = json["height"].as_f64();
        let option_r = json["radius"].as_f64();

        Box::new(
            Cylinder {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                height: option_h.unwrap_or(2.0),
                radius: option_r.unwrap_or(1.0),
            }
        )
    }

    pub fn get_cone_from_json(&self, json: &Value) -> Box::<Cone> {
        let option_h = json["height"].as_f64();
        let option_r = json["radius"].as_f64();

        Box::new(
            Cone {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
                height: option_h.unwrap_or(3.0),
                radius: option_r.unwrap_or(1.0),
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

    pub fn get_directional_from_json(&self, json: &Value) -> Box::<Directional> {
        let option_s = json["strength"].as_f64();

        Box::new(
            Directional {
                transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
                color: if json["color"].is_object() {self.get_color_from_json(&json["transform"])} else {Color::default()},
                strength: option_s.unwrap_or(1000.0),
            }
        )
    }

    pub fn get_object_lights_from_json(&self, json: &Value) -> Vec::<Box::<dyn Light>> {
        let mut lights: Vec::<Box::<dyn Light>> = Vec::new();

        if json["directional"].is_array(){
            for directional in json["directional"].as_array().unwrap().iter() {
                lights.push(self.get_directional_from_json(directional))
            }
        }
        lights
    }

    pub fn get_ambiant_from_json(&self, json: &Value) -> Ambiant {
        let option_s = json["strength"].as_f64();
        let colval: Color;

        if json["color"].is_object() {colval = self.get_color_from_json(&json["transform"]);}
        else {colval = Color::default();}
        Ambiant {
            color: colval,
            strength: option_s.unwrap_or(1000.0),
        }
    }

    pub fn get_ambiants_from_json(&self, json: &Value) -> Vec::<Ambiant> {
        let mut lights: Vec::<Ambiant> = Vec::new();

        if json["ambiant"].is_array() {
            for ambiant in json["ambiant"].as_array().unwrap().iter() {
                lights.push(self.get_ambiant_from_json(ambiant))
            }
        }
        lights
    }

    pub fn get_lights_from_json(&self, json: &Value) -> Lights {
        let objval : Vec::<Box::<dyn Light>>;

        if json["objects"].is_object() {objval = self.get_object_lights_from_json(&json["objects"]);}
        else {objval = Vec::new()}
        Lights {
            lights: objval,
            ambiant: self.get_ambiants_from_json(&json),
        }
    }

}
