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

    pub fn get_vector_from_json(&self, json: &Value) -> Vector{
        let optionx = json["x"].as_f64();
        let optiony = json["y"].as_f64();
        let optionz = json["z"].as_f64();
        let xval : f64;
        let yval : f64;
        let zval : f64;
        if optionx != None {xval = optionx.unwrap();}
        else {xval = 0.0;}
        if optiony != None {yval = optiony.unwrap();}
        else {yval = 0.0;}
        if optionz != None {zval = optionz.unwrap();}
        else {zval = 0.0;}
        let vector = Vector {
            x: xval,
            y: yval,
            z: zval,
        };
        vector
    }

    pub fn get_transform_from_json(&self, json: &Value) -> Transform {
        let transform = Transform {
            pos: self.get_vector_from_json(&json["pos"]),
            rotation: self.get_vector_from_json(&json["rotation"]),
            scale: self.get_vector_from_json(&json["scale"])
        };
        transform
    }

    pub fn get_lens_from_json(&self, json: &Value) -> Lens {
        let lens = Lens {
            height: json["height"].as_i64().unwrap(),
            width: json["width"].as_i64().unwrap(),
            distance: json["distance"].as_f64().unwrap(),
            vector_to_first_pixel: self.get_vector_from_json(&json["vector_to_first_pixel"]),
        };
        lens
    }

    pub fn get_camera_from_json(&self, json: &Value) -> Camera {
        let mut camera = Camera {
            transform: self.get_transform_from_json(&json["transform"]),
            lens: self.get_lens_from_json(&json["lens"]),
            fov: json["fov"].as_i64().unwrap(),
            diffuse: json["diffuse"].as_f64().unwrap(),
            ambient: json["ambiant"].as_f64().unwrap(),
            specular: json["specular"].as_f64().unwrap(),
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
        let color = Color {
            r: (json["r"].as_f64().unwrap() % 255.0),
            g: (json["g"].as_f64().unwrap() % 255.0),
            b: (json["b"].as_f64().unwrap() % 255.0),
        };
        color
    }

    pub fn get_texture_from_json(&self, json: &Value) -> Texture {
        let texture = Texture {
            texture_type: json["texture_type"].as_u64().unwrap(),
            color: self.get_color_from_json(&json["color"]),
            diffuse: json["diffuse"].as_f64().unwrap(),
            ambient: json["ambient"].as_f64().unwrap(),
            specular: json["specular"].as_f64().unwrap(),
            shininess: json["shininess"].as_f64().unwrap(),
        };
        texture
    }

    pub fn get_sphere_from_json(&self, json: &Value) -> Box::<Sphere> {
        let sphere = Sphere {
            transform: self.get_transform_from_json(&json["transform"]),
            texture: self.get_texture_from_json(&json["texture"]),
            radius: json["radius"].as_f64().unwrap(),
        };
        let spherebox = Box::new(sphere);
        spherebox
    }

    pub fn get_plane_from_json(&self, json: &Value) -> Box::<Plane> {
        let plane = Plane {
            transform: self.get_transform_from_json(&json["transform"]),
            texture: self.get_texture_from_json(&json["texture"]),
            origin: self.get_vector_from_json(&json["origin"]),
            vector: self.get_vector_from_json(&json["vector"]),
        };
        let planebox = Box::new(plane);
        planebox
    }

    pub fn get_cylinder_from_json(&self, json: &Value) -> Box::<Cylinder> {
        let cylinder = Cylinder {
            transform: self.get_transform_from_json(&json["transform"]),
            texture: self.get_texture_from_json(&json["texture"]),
            height: json["height"].as_f64().unwrap(),
            radius: json["radius"].as_f64().unwrap(),
        };
        let cylinderbox = Box::new(cylinder);
        cylinderbox
    }

    pub fn get_cone_from_json(&self, json: &Value) -> Box::<Cone> {
        let cone = Cone {
            transform: self.get_transform_from_json(&json["transform"]),
            texture: self.get_texture_from_json(&json["texture"]),
            height: json["height"].as_f64().unwrap(),
            radius: json["radius"].as_f64().unwrap(),
        };
        let conebox = Box::new(cone);
        conebox
    }

    pub fn get_objects_from_json(&self, json: &Value) -> Vec::<Box::<dyn Object>> {
        let mut objects: Vec::<Box::<dyn Object>> = Vec::new();
        for sphere in json["spheres"].as_array().unwrap().iter() {
            objects.push(self.get_sphere_from_json(sphere))
        }
        for plane in json["planes"].as_array().unwrap().iter() {
            objects.push(self.get_plane_from_json(plane))
        }
        for cylinder in json["cylinders"].as_array().unwrap().iter() {
            objects.push(self.get_cylinder_from_json(cylinder))
        }
        for cone in json["cones"].as_array().unwrap().iter() {
            objects.push(self.get_cone_from_json(cone))
        }
        objects
    }

    pub fn get_directional_from_json(&self, json: &Value) -> Box::<Directional> {
        let directional = Directional {
            transform: self.get_transform_from_json(&json["transform"]),
            color: self.get_color_from_json(&json["color"]),
            strength: json["strength"].as_f64().unwrap(),
        };
        let directionalbox = Box::new(directional);
        directionalbox
    }

    pub fn get_object_lights_from_json(&self, json: &Value) -> Vec::<Box::<dyn Light>> {
        let mut lights: Vec::<Box::<dyn Light>> = Vec::new();
        for directional in json["directional"].as_array().unwrap().iter() {
            lights.push(self.get_directional_from_json(directional))
        }
        lights
    }

    pub fn get_ambiant_from_json(&self, json: &Value) -> Ambiant {
        let ambiant = Ambiant {
            color: self.get_color_from_json(&json["color"]),
            strength: json["strength"].as_f64().unwrap(),
        };
        ambiant
    }

    pub fn get_ambiants_from_json(&self, json: &Value) -> Vec::<Ambiant> {
        let mut lights: Vec::<Ambiant> = Vec::new();
        for ambiant in json["ambiant"].as_array().unwrap().iter() {
            lights.push(self.get_ambiant_from_json(ambiant))
        }
        lights
    }

    pub fn get_lights_from_json(&self, json: &Value) -> Lights {
        let lights = Lights {
            lights: self.get_object_lights_from_json(&json["objects"]),
            ambiant: self.get_ambiants_from_json(&json),
        };
        lights
    }

}
