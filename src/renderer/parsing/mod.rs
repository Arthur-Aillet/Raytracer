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
        let ispos = json["pos"].is_object();
        let isrot = json["rotation"].is_object();
        let isscale = json["scale"].is_object();
        let posval: Vector;
        let rotval: Vector;
        let scaleval: Vector;

        if ispos {posval = self.get_vector_from_json(&json["pos"]);}
        else {posval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        if isrot {rotval = self.get_vector_from_json(&json["rotation"]);}
        else {rotval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        if isscale {scaleval = self.get_vector_from_json(&json["scale"]);}
        else {scaleval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        let transform = Transform {
            pos: posval,
            rotation: rotval,
            scale:  scaleval,
        };
        transform
    }

    pub fn get_lens_from_json(&self, json: &Value) -> Lens {
        let optionh = json["height"].as_i64();
        let optionw = json["width"].as_i64();
        let optiond =json["distance"].as_f64();
        let isvec = json["vector_to_first_pixel"].is_object();
        let hval : i64;
        let wval: i64;
        let dval: f64;
        let vecval: Vector;

        if optionh != None {hval = optionh.unwrap();}
        else {hval = 1080;}
        if optionw != None {wval = optionw.unwrap();}
        else {wval = 1920;}
        if optiond != None {dval = optiond.unwrap();}
        else {dval = 0.0;}
        if isvec {vecval = self.get_vector_from_json(&json["vector_to_first_pixel"]);}
        else {vecval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        let lens = Lens {
            height: hval,
            width: wval,
            distance: dval,
            vector_to_first_pixel: vecval,
        };
        lens
    }

    pub fn get_camera_from_json(&self, json: &Value) -> Camera {
        let istrans = json["transform"].is_object();
        let islens = json["lens"].is_object();
        let optionf = json["fov"].as_i64();
        let optiond = json["diffuse"].as_f64();
        let optiona = json["ambient"].as_f64();
        let options = json["specular"].as_f64();
        let transval : Transform;
        let lensval : Lens;
        let fval: i64;
        let dval: f64;
        let aval: f64;
        let sval: f64;

        if istrans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if islens {lensval = self.get_lens_from_json(&json["lens"]);}
        else {lensval = Lens::default();}
        if optionf != None {fval = optionf.unwrap();}
        else {fval = 60;}
        if optiond != None {dval = optiond.unwrap();}
        else {dval = 0.7;}
        if optiona != None {aval = optiona.unwrap();}
        else {aval = 0.1;}
        if options != None {sval = options.unwrap();}
        else {sval = 0.6;}
        let mut camera = Camera {
            transform: transval,
            lens: lensval,
            fov: fval,
            diffuse: dval,
            ambient: aval,
            specular: sval,
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
        let optionr = json["r"].as_f64();
        let optiong = json["g"].as_f64();
        let optionb = json["b"].as_f64();
        let rval : f64;
        let gval : f64;
        let bval : f64;

        if optionr != None {rval = optionr.unwrap();}
        else {rval = 255.0;}
        if optiong != None {gval = optiong.unwrap();}
        else {gval = 255.0;}
        if optionb != None {bval = optionb.unwrap();}
        else {bval = 255.0;}
        let color = Color {
            r: (rval % 256.0),
            g: (gval % 256.0),
            b: (bval % 256.0),
        };
        color
    }

    pub fn get_texture_from_json(&self, json: &Value) -> Texture {
        let optiont = json["texture_type"].as_u64();
        let iscol = json["color"].is_object();
        let optiond = json["diffuse"].as_f64();
        let optiona = json["ambient"].as_f64();
        let optionsp = json["specular"].as_f64();
        let optionsh = json["shininess"].as_f64();
        let tval: u64;
        let colval: Color;
        let dval: f64;
        let aval: f64;
        let spval: f64;
        let shval: f64;

        if optiont != None {tval = optiont.unwrap();}
        else {tval = 1;}
        if iscol {colval = self.get_color_from_json(&json["color"]);}
        else {colval = Color::default();}
        if optiond != None {dval = optiond.unwrap();}
        else {dval = 0.7;}
        if optiona != None {aval = optiona.unwrap();}
        else {aval = 0.1;}
        if optionsp != None {spval = optionsp.unwrap();}
        else {spval = 0.4;}
        if optionsh != None {shval = optionsh.unwrap();}
        else {shval = 4.0;}
        let texture = Texture {
            texture_type: tval,
            color: colval,
            diffuse: dval,
            ambient: aval,
            specular: spval,
            shininess: shval,
        };

        texture
    }

    pub fn get_sphere_from_json(&self, json: &Value) -> Box::<Sphere> {
        let istrans = json["transform"].is_object();
        let istex = json["texture"].is_object();
        let optionr = json["radius"].as_f64();
        let transval : Transform;
        let texval : Texture;
        let rval : f64;

        if istrans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if istex {texval = self.get_texture_from_json(&json["texture"]);}
        else {texval = Texture::default();}
        if optionr != None {rval = optionr.unwrap();}
        else {rval = 1.0;}
        let sphere = Sphere {
            transform: transval,
            texture: texval,
            radius: rval,
        };
        let spherebox = Box::new(sphere);
        spherebox
    }

    pub fn get_plane_from_json(&self, json: &Value) -> Box::<Plane> {
        let istrans = json["transform"].is_object();
        let istex = json["texture"].is_object();
        let isorg = json["origin"].is_object();
        let isvec = json["vector"].is_object();
        let transval : Transform;
        let texval : Texture;
        let orgval : Vector;
        let vecval : Vector;
        if istrans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if istex {texval = self.get_texture_from_json(&json["texture"]);}
        else {texval = Texture::default();}
        if isorg {orgval = self.get_vector_from_json(&json["transform"]);}
        else {orgval = Vector {x: 0.0, y: 0.0, z: 0.0};}
        if isvec {vecval = self.get_vector_from_json(&json["transform"]);}
        else {vecval = Vector {x: 0.0, y: 1.0, z: 0.0};}
        let plane = Plane {
            transform: transval,
            texture: texval,
            origin: orgval,
            vector: vecval,
        };
        let planebox = Box::new(plane);
        planebox
    }

    pub fn get_cylinder_from_json(&self, json: &Value) -> Box::<Cylinder> {
        let istrans = json["transform"].is_object();
        let istex = json["texture"].is_object();
        let optionh = json["height"].as_f64();
        let optionr = json["radius"].as_f64();
        let transval : Transform;
        let texval : Texture;
        let hval: f64;
        let rval: f64;

        if istrans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if istex {texval = self.get_texture_from_json(&json["texture"]);}
        else {texval = Texture::default();}
        if optionh != None {hval = optionh.unwrap();}
        else {hval = 2.0;}
        if optionr != None {rval = optionr.unwrap();}
        else {rval = 1.0;}
        let cylinder = Cylinder {
            transform: transval,
            texture: texval,
            height: hval,
            radius: rval,
        };
        let cylinderbox = Box::new(cylinder);
        cylinderbox
    }

    pub fn get_cone_from_json(&self, json: &Value) -> Box::<Cone> {
        let istrans = json["transform"].is_object();
        let istex = json["texture"].is_object();
        let optionh = json["height"].as_f64();
        let optionr = json["radius"].as_f64();
        let transval: Transform;
        let texval: Texture;
        let hval: f64;
        let rval: f64;

        if istrans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if istex {texval = self.get_texture_from_json(&json["texture"]);}
        else {texval = Texture::default();}
        if optionh != None {hval = optionh.unwrap();}
        else {hval = 3.0;}
        if optionr != None {rval = optionr.unwrap();}
        else {rval = 1.0;}
        let cone = Cone {
            transform: transval,
            texture: texval,
            height: hval,
            radius: rval,
        };
        let conebox = Box::new(cone);
        conebox
    }

    pub fn get_objects_from_json(&self, json: &Value) -> Vec::<Box::<dyn Object>> {
        let mut objects: Vec::<Box::<dyn Object>> = Vec::new();
        let issph = json["spheres"].is_array();
        let ispla = json["planes"].is_array();
        let iscyl = json["cylinders"].is_array();
        let iscon = json["cones"].is_array();

        if issph {
            for sphere in json["spheres"].as_array().unwrap().iter() {
                objects.push(self.get_sphere_from_json(sphere))
            }
        } else {}
        if ispla {
            for plane in json["planes"].as_array().unwrap().iter() {
                objects.push(self.get_plane_from_json(plane))
            }
        } else {}
        if iscyl {
            for cylinder in json["cylinders"].as_array().unwrap().iter() {
                objects.push(self.get_cylinder_from_json(cylinder))
            }
        } else {}
        if iscon {
            for cone in json["cones"].as_array().unwrap().iter() {
                objects.push(self.get_cone_from_json(cone))
            }
        } else {}
        objects
    }

    pub fn get_directional_from_json(&self, json: &Value) -> Box::<Directional> {
        let istrans = json["transform"].is_object();
        let iscol = json["color"].is_object();
        let options = json["strength"].as_f64();
        let transval: Transform;
        let colval: Color;
        let sval: f64;

        if istrans {transval = self.get_transform_from_json(&json["transform"]);}
        else {transval = Transform::default();}
        if iscol {colval = self.get_color_from_json(&json["transform"]);}
        else {colval = Color::default();}
        if options != None {sval = options.unwrap();}
        else {sval = 1000.0;}
        let directional = Directional {
            transform: transval,
            color: colval,
            strength: sval,
        };
        let directionalbox = Box::new(directional);
        directionalbox
    }

    pub fn get_object_lights_from_json(&self, json: &Value) -> Vec::<Box::<dyn Light>> {
        let mut lights: Vec::<Box::<dyn Light>> = Vec::new();
        let isdir = json["directional"].is_array();

        if isdir {
            for directional in json["directional"].as_array().unwrap().iter() {
                lights.push(self.get_directional_from_json(directional))
            }
        } else {}
        lights
    }

    pub fn get_ambiant_from_json(&self, json: &Value) -> Ambiant {
        let iscol = json["color"].is_object();
        let options = json["strength"].as_f64();
        let colval: Color;
        let sval: f64;

        if iscol {colval = self.get_color_from_json(&json["transform"]);}
        else {colval = Color::default();}
        if options != None {sval = options.unwrap();}
        else {sval = 1000.0;}
        let ambiant = Ambiant {
            color: colval,
            strength: sval,
        };
        ambiant
    }

    pub fn get_ambiants_from_json(&self, json: &Value) -> Vec::<Ambiant> {
        let mut lights: Vec::<Ambiant> = Vec::new();
        let isamb = json["ambiant"].is_array();

        if isamb {
            for ambiant in json["ambiant"].as_array().unwrap().iter() {
                lights.push(self.get_ambiant_from_json(ambiant))
            }
        } else {}
        lights
    }

    pub fn get_lights_from_json(&self, json: &Value) -> Lights {
        let isobj = json["objects"].is_object();
        let objval : Vec::<Box::<dyn Light>>;

        if isobj {objval = self.get_object_lights_from_json(&json["objects"]);}
        else {objval = Vec::new()}
        let lights = Lights {
            lights: objval,
            ambiant: self.get_ambiants_from_json(&json),
        };
        lights
    }

}
