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
        let is_pos = json["pos"].is_object();
        let is_rot = json["rotation"].is_object();
        let is_scale = json["scale"].is_object();
        let posval: Vector;
        let rotval: Vector;
        let scaleval: Vector;


        if is_pos {posval = self.get_vector_from_json(&json["pos"]);}
        else {posval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        if is_rot {rotval = self.get_vector_from_json(&json["rotation"]);}
        else {rotval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        if is_scale {scaleval = self.get_vector_from_json(&json["scale"]);}
        else {scaleval = Vector {x: 1.0, y: 1.0, z: 1.0};}
        let transform = Transform {
            pos: posval,
            rotation: rotval,
            scale:  scaleval,
        };
        transform
    }

    pub fn get_lens_from_json(&self, json: &Value) -> Lens {
        let option_h = json["height"].as_i64();
        let option_w = json["width"].as_i64();
        let option_d =json["distance"].as_f64();
        let is_vec = json["vector_to_first_pixel"].is_object();
        let vecval: Vector;

        if is_vec {vecval = self.get_vector_from_json(&json["vector_to_first_pixel"]);}
        else {vecval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        let lens = Lens {
            height: option_h.unwrap_or(0),
            width: option_w.unwrap_or(0),
            distance: option_d.unwrap_or(0.0),
            vector_to_first_pixel: vecval,
        };
        lens
    }

    pub fn get_camera_from_json(&self, json: &Value) -> Camera {
        let is_trans = json["transform"].is_object();
        let is_lens = json["lens"].is_object();
        let option_f = json["fov"].as_i64();
        let option_d = json["diffuse"].as_f64();
        let option_a = json["ambient"].as_f64();
        let option_s = json["specular"].as_f64();
        let transval : Transform;
        let lensval : Lens;

        if is_trans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if is_lens {lensval = self.get_lens_from_json(&json["lens"]);}
        else {lensval = Lens::default();}
        let mut camera = Camera {
            transform: transval,
            lens: lensval,
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

        let color = Color {
            r: (option_r.unwrap_or(255.0) % 256.0),
            g: (option_g.unwrap_or(255.0) % 256.0),
            b: (option_b.unwrap_or(255.0) % 256.0),
        };
        color
    }

    pub fn get_texture_from_json(&self, json: &Value) -> Texture {
        let option_t = json["texture_type"].as_u64();
        let is_col = json["color"].is_object();
        let option_d = json["diffuse"].as_f64();
        let option_a = json["ambient"].as_f64();
        let option_sp = json["specular"].as_f64();
        let option_sh = json["shininess"].as_f64();
        let colval: Color;

        if is_col {colval = self.get_color_from_json(&json["color"]);}
        else {colval = Color::default();}
        let texture = Texture {
            texture_type: option_t.unwrap_or(1),
            color: colval,
            diffuse: option_d.unwrap_or(0.7),
            ambient: option_a.unwrap_or(0.1),
            specular: option_sp.unwrap_or(0.4),
            shininess: option_sh.unwrap_or(4.0),
        };

        texture
    }

    pub fn get_sphere_from_json(&self, json: &Value) -> Box::<Sphere> {
        let is_trans = json["transform"].is_object();
        let is_tex = json["texture"].is_object();
        let option_r = json["radius"].as_f64();
        let transval : Transform;
        let texval : Texture;

        if is_trans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if is_tex {texval = self.get_texture_from_json(&json["texture"]);}
        else {texval = Texture::default();}
        let sphere = Sphere {
            transform: transval,
            texture: texval,
            radius: option_r.unwrap_or(1.0),
        };
        Box::new(sphere)
    }

    pub fn get_plane_from_json(&self, json: &Value) -> Box::<Plane> {
        let is_tex = json["texture"].is_object();
        let is_norm = json["normal"].is_object();
        let option_d = json["vector"].as_f64();
        let texval : Texture;
        let normval : Vector;

        if is_tex {texval = self.get_texture_from_json(&json["texture"]);}
        else {texval = Texture::default();}
        if is_norm {normval = self.get_vector_from_json(&json["normal"]);}
        else {normval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        let plane = Plane {
            texture: texval,
            normal: normval,
            distance: option_d.unwrap_or(0.0),
        };
        Box::new(plane)
    }

    pub fn get_cylinder_from_json(&self, json: &Value) -> Box::<Cylinder> {
        let is_trans = json["transform"].is_object();
        let is_tex = json["texture"].is_object();
        let option_h = json["height"].as_f64();
        let option_r = json["radius"].as_f64();
        let transval : Transform;
        let texval : Texture;

        if is_trans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if is_tex {texval = self.get_texture_from_json(&json["texture"]);}
        else {texval = Texture::default();}
        let cylinder = Cylinder {
            transform: transval,
            texture: texval,
            height: option_h.unwrap_or(2.0),
            radius: option_r.unwrap_or(1.0),
        };
        Box::new(cylinder)
    }

    pub fn get_cone_from_json(&self, json: &Value) -> Box::<Cone> {
        let is_trans = json["transform"].is_object();
        let is_tex = json["texture"].is_object();
        let option_h = json["height"].as_f64();
        let option_r = json["radius"].as_f64();
        let transval: Transform;
        let texval: Texture;

        if is_trans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if is_tex {texval = self.get_texture_from_json(&json["texture"]);}
        else {texval = Texture::default();}
        let cone = Cone {
            transform: transval,
            texture: texval,
            height: option_h.unwrap_or(3.0),
            radius: option_r.unwrap_or(1.0),
        };
        Box::new(cone)
    }

    pub fn get_objects_from_json(&self, json: &Value) -> Vec::<Box::<dyn Object>> {
        let mut objects: Vec::<Box::<dyn Object>> = Vec::new();
        let is_sph = json["spheres"].is_array();
        let is_pla = json["planes"].is_array();
        let is_cyl = json["cylinders"].is_array();
        let is_con = json["cones"].is_array();

        if is_sph {
            for sphere in json["spheres"].as_array().unwrap().iter() {
                objects.push(self.get_sphere_from_json(sphere))
            }
        }
        if is_pla {
            for plane in json["planes"].as_array().unwrap().iter() {
                objects.push(self.get_plane_from_json(plane))
            }
        }
        if is_cyl {
            for cylinder in json["cylinders"].as_array().unwrap().iter() {
                objects.push(self.get_cylinder_from_json(cylinder))
            }
        }
        if is_con {
            for cone in json["cones"].as_array().unwrap().iter() {
                objects.push(self.get_cone_from_json(cone))
            }
        }
        objects
    }

    pub fn get_directional_from_json(&self, json: &Value) -> Box::<Directional> {
        let is_trans = json["transform"].is_object();
        let is_col = json["color"].is_object();
        let option_s = json["strength"].as_f64();
        let transval: Transform;
        let colval: Color;

        if is_trans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if is_col {colval = self.get_color_from_json(&json["transform"]);}
        else {colval = Color::default();}
        let directional = Directional {
            transform: transval,
            color: colval,
            strength: option_s.unwrap_or(1000.0),
        };
        Box::new(directional)
    }

    pub fn get_object_lights_from_json(&self, json: &Value) -> Vec::<Box::<dyn Light>> {
        let mut lights: Vec::<Box::<dyn Light>> = Vec::new();
        let isdir = json["directional"].is_array();

        if isdir {
            for directional in json["directional"].as_array().unwrap().iter() {
                lights.push(self.get_directional_from_json(directional))
            }
        }
        lights
    }

    pub fn get_ambiant_from_json(&self, json: &Value) -> Ambiant {
        let is_col = json["color"].is_object();
        let option_s = json["strength"].as_f64();
        let colval: Color;

        if is_col {colval = self.get_color_from_json(&json["transform"]);}
        else {colval = Color::default();}
        let ambiant = Ambiant {
            color: colval,
            strength: option_s.unwrap_or(1000.0),
        };
        ambiant
    }

    pub fn get_ambiants_from_json(&self, json: &Value) -> Vec::<Ambiant> {
        let mut lights: Vec::<Ambiant> = Vec::new();
        let is_amb = json["ambiant"].is_array();

        if is_amb {
            for ambiant in json["ambiant"].as_array().unwrap().iter() {
                lights.push(self.get_ambiant_from_json(ambiant))
            }
        }
        lights
    }

    pub fn get_lights_from_json(&self, json: &Value) -> Lights {
        let is_obj = json["objects"].is_object();
        let objval : Vec::<Box::<dyn Light>>;

        if is_obj {objval = self.get_object_lights_from_json(&json["objects"]);}
        else {objval = Vec::new()}
        let lights = Lights {
            lights: objval,
            ambiant: self.get_ambiants_from_json(&json),
        };
        lights
    }

}
