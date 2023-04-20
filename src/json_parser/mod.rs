//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// mod
//

use std::fs;
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Point,
    pub rotation: Point,
    pub scale: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub transform: Transform,
    pub focal_lenght: f64,
    pub height: f64,
    pub width: f64,
    pub fov: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    pub texture_type: u64,
    pub color: Color,
}

impl Texture {
    fn texture(&self, _x: f64, _y: f64) -> Color {
        if self.texture_type == 1 {
            self.color
        } else {self.color}
    }
}

pub struct Sphere {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
}

pub struct Plane {
    pub transform: Transform,
    pub texture: Texture,
    pub origin: Point,
    pub vector: Point,
}
pub struct Cylinder {
    pub transform: Transform,
    pub texture: Texture,
    pub height: f64,
    pub radius: f64,
}

pub struct Cone {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
    pub height: f64,
}

trait Object {
    fn obj_type(&self) -> String;
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
    fn get_texture(&self) -> Texture;
    fn set_texture(&mut self, new: Texture);
    fn get_radius(&self) -> f64;
    fn set_radius(&mut self, new: f64);
    fn get_height(&self) -> f64;
    fn set_height(&mut self, new: f64);
    fn get_origin(&self) -> Point;
    fn set_origin(&mut self, new: Point);
    fn get_vector(&self) -> Point;
    fn set_vector(&mut self, new: Point);
    fn slanted_height(&self) -> f64;
    fn diameter(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn surface(&self) -> f64;
    fn lateral_surface(&self) -> f64;
    fn base_surface(&self) -> f64;
    fn volume(&self) -> f64;
}

impl Object for Sphere {
    fn obj_type(&self) -> String {format!("sphere")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn diameter(&self) -> f64 {self.radius * 2.0}
    fn perimeter(&self) -> f64 {self.radius * std::f64::consts::PI * 2.0}
    fn surface(&self) -> f64 {self.radius.powi(2) * std::f64::consts::PI * 4.0}
    fn volume(&self) -> f64 {((std::f64::consts::PI * self.radius.powi(3)) * 4.0) / 3.0}

    fn get_height(&self) -> f64 {1.0}
    fn set_height(&mut self, _new: f64) {}
    fn get_origin(&self) -> Point {
        let a = Point{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Point) {}
    fn get_vector(&self) -> Point {
        let a = Point{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Point) {}
    fn slanted_height(&self) -> f64 {1.0}
    fn lateral_surface(&self) -> f64 {self.surface()}
    fn base_surface(&self) -> f64 {self.surface()}
}

impl Object for Plane {
    fn obj_type(&self) -> String {format!("plane")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_origin(&self) -> Point {self.origin}
    fn set_origin(&mut self, new: Point) {self.origin = new}
    fn get_vector(&self) -> Point {self.vector}
    fn set_vector(&mut self, new: Point) {self.vector = new}

    fn get_radius(&self) -> f64 {1.0}
    fn set_radius(&mut self, _new: f64) {}
    fn get_height(&self) -> f64 {1.0}
    fn set_height(&mut self, _new: f64) {}
    fn slanted_height(&self) -> f64 {1.0}
    fn diameter(&self) -> f64 {1.0}
    fn perimeter(&self) -> f64 {1.0}
    fn surface(&self) -> f64 {1.0}
    fn lateral_surface(&self) -> f64 {1.0}
    fn base_surface(&self) -> f64 {1.0}
    fn volume(&self) -> f64 {1.0}
}

impl Object for Cylinder {
    fn obj_type(&self) -> String {format!("cylinder")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn get_height(&self) -> f64 {self.height}
    fn set_height(&mut self, new: f64) {self.height = new}
    fn diameter(&self) -> f64 {self.radius * 2.0}
    fn perimeter(&self) -> f64 {self.radius * std::f64::consts::PI * 2.0}
    fn surface(&self) -> f64 {2.0 * self.base_surface() + self.lateral_surface()}
    fn lateral_surface(&self) -> f64 {2.0 * std::f64::consts::PI * self.radius * self.height}
    fn base_surface(&self) -> f64 {std::f64::consts::PI * self.radius.powi(2)}
    fn volume(&self) -> f64 {std::f64::consts::PI * self.radius.powi(2) * self.height}

    fn get_origin(&self) -> Point {
        let a = Point{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Point) {}
    fn get_vector(&self) -> Point {
        let a = Point{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Point) {}
    fn slanted_height(&self) -> f64 {1.0}
}

impl Object for Cone {
    fn obj_type(&self) -> String {format!("cone")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_radius(&self) -> f64 {self.radius}
    fn set_radius(&mut self, new: f64) {self.radius = new}
    fn get_height(&self) -> f64 {self.height}
    fn set_height(&mut self, new: f64) {self.height = new}
    fn slanted_height(&self) -> f64 {(self.radius.powi(2) + self.height.powi(2)).sqrt()}
    fn diameter(&self) -> f64 {self.radius * 2.0}
    fn perimeter(&self) -> f64 {self.radius * std::f64::consts::PI * 2.0}
    fn surface(&self) -> f64 {self.lateral_surface() + self.base_surface()}
    fn lateral_surface(&self) -> f64 {std::f64::consts::PI * self.radius * self.slanted_height()}
    fn base_surface(&self) -> f64 {std::f64::consts::PI * self.radius.powi(2)}
    fn volume(&self) -> f64 {(std::f64::consts::PI * self.radius.powi(2) * self.height) / 3.0}

    fn get_origin(&self) -> Point {
        let a = Point{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Point) {}
    fn get_vector(&self) -> Point {
        let a = Point{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Point) {}
}
pub struct Directional {
    pub transform: Transform,
    pub color: Color,
    pub strength: f64,
}

trait Light {
    fn light_type(&self) -> String;
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
    fn get_color(&self) -> Color;
    fn set_color(&mut self, new: Color);
    fn get_strength(&self) -> f64;
    fn set_strength(&mut self, new: f64);
}

impl Light for Directional {
    fn light_type(&self) -> String {format!("directional")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_color(&self) -> Color {self.color}
    fn set_color(&mut self, new: Color) {self.color = new}
    fn get_strength(&self) -> f64 {self.strength}
    fn set_strength(&mut self, new: f64) {self.strength = new}
}

pub struct Ambiant {
    pub color: Color,
    pub strength: f64,
}

pub struct Lights {
    pub lights: Vec::<Box::<dyn Light>>,
    pub ambiant: Vec<Ambiant>,
}

pub struct Renderer {
    pub camera: Camera,
    pub primitives: Vec::<Box::<dyn Object>>,
    pub lights: Lights,
}

impl Renderer {

    pub fn new() -> Renderer {
        let renderer = Renderer {
            camera: Camera {
                transform: Transform {
                    pos: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    },
                    rotation: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    },
                    scale: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    }
                },
                focal_lenght: 0.0,
                height: 0.0,
                width: 0.0,
                fov: 0.0
            },
            primitives: Vec::new(),
            lights: Lights {
                lights: Vec::new(),
                ambiant: Vec::new(),
            },
        };
        renderer
    }

    fn get_point_from_json(&self, json: &Value) -> Point{
        let point = Point {
            x: json["x"].as_f64().unwrap(),
            y: json["y"].as_f64().unwrap(),
            z: json["z"].as_f64().unwrap()
        };
        point
    }

    fn get_transform_from_json(&self, json: &Value) -> Transform {
        let transform = Transform {
            pos: self.get_point_from_json(&json["pos"]),
            rotation: self.get_point_from_json(&json["rotation"]),
            scale: self.get_point_from_json(&json["scale"])
        };
        transform
    }

    fn get_camera_from_json(&self, json: &Value) -> Camera {
        let camera = Camera {
            transform: self.get_transform_from_json(&json["transform"]),
            focal_lenght: json["focal_lenght"].as_f64().unwrap(),
            height: json["height"].as_f64().unwrap(),
            width: json["width"].as_f64().unwrap(),
            fov: json["fov"].as_f64().unwrap(),
        };
        camera
    }

    fn get_color_from_json(&self, json: &Value) -> Color {
        let color = Color {
            r: (json["r"].as_f64().unwrap() % 255.0) / 255.0,
            g: (json["g"].as_f64().unwrap() % 255.0) / 255.0,
            b: (json["b"].as_f64().unwrap() % 255.0) / 255.0,
        };
        color
    }

    fn get_texture_from_json(&self, json: &Value) -> Texture {
        let texture = Texture {
            texture_type: json["texture_type"].as_u64().unwrap(),
            color: self.get_color_from_json(&json["color"]),
        };
        texture
    }

    fn get_sphere_from_json(&self, json: &Value) -> Box::<Sphere> {
        let sphere = Sphere {
            transform: self.get_transform_from_json(&json["transform"]),
            texture: self.get_texture_from_json(&json["texture"]),
            radius: json["radius"].as_f64().unwrap(),
        };
        let spherebox = Box::new(sphere);
        spherebox
    }

    fn get_plane_from_json(&self, json: &Value) -> Box::<Plane> {
        let plane = Plane {
            transform: self.get_transform_from_json(&json["transform"]),
            texture: self.get_texture_from_json(&json["texture"]),
            origin: self.get_point_from_json(&json["origin"]),
            vector: self.get_point_from_json(&json["vector"]),
        };
        let planebox = Box::new(plane);
        planebox
    }

    fn get_cylinder_from_json(&self, json: &Value) -> Box::<Cylinder> {
        let cylinder = Cylinder {
            transform: self.get_transform_from_json(&json["transform"]),
            texture: self.get_texture_from_json(&json["texture"]),
            height: json["height"].as_f64().unwrap(),
            radius: json["radius"].as_f64().unwrap(),
        };
        let cylinderbox = Box::new(cylinder);
        cylinderbox
    }

    fn get_cone_from_json(&self, json: &Value) -> Box::<Cone> {
        let cone = Cone {
            transform: self.get_transform_from_json(&json["transform"]),
            texture: self.get_texture_from_json(&json["texture"]),
            height: json["height"].as_f64().unwrap(),
            radius: json["radius"].as_f64().unwrap(),
        };
        let conebox = Box::new(cone);
        conebox
    }

    fn get_objects_from_json(&self, json: &Value) -> Vec::<Box::<dyn Object>> {
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

    fn get_directional_from_json(&self, json: &Value) -> Box::<Directional> {
        let directional = Directional {
            transform: self.get_transform_from_json(&json["transform"]),
            color: self.get_color_from_json(&json["color"]),
            strength: json["strength"].as_f64().unwrap(),
        };
        let directionalbox = Box::new(directional);
        directionalbox
    }

    fn get_object_lights_from_json(&self, json: &Value) -> Vec::<Box::<dyn Light>> {
        let mut lights: Vec::<Box::<dyn Light>> = Vec::new();
        for directional in json["directional"].as_array().unwrap().iter() {
            lights.push(self.get_directional_from_json(directional))
        }
        lights
    }

    fn get_ambiant_from_json(&self, json: &Value) -> Ambiant {
        let ambiant = Ambiant {
            color: self.get_color_from_json(&json["color"]),
            strength: json["strength"].as_f64().unwrap(),
        };
        ambiant
    }

    fn get_ambiants_from_json(&self, json: &Value) -> Vec::<Ambiant> {
        let mut lights: Vec::<Ambiant> = Vec::new();
        for ambiant in json["ambiant"].as_array().unwrap().iter() {
            lights.push(self.get_ambiant_from_json(ambiant))
        }
        lights
    }

    fn get_lights_from_json(&self, json: &Value) -> Lights {
        let lights = Lights {
            lights: self.get_object_lights_from_json(&json["objects"]),
            ambiant: self.get_ambiants_from_json(&json),
        };
        lights
    }

    pub fn get_renderer_from_file(&mut self, file: String) {
        let data = fs::read_to_string(file).expect("Unable to read file");
        let json: Value = serde_json::from_str(&data.to_string()).unwrap();
        self.camera = self.get_camera_from_json(&json["camera"]);
        self.primitives = self.get_objects_from_json(&json["primitives"]);
        self.lights = self.get_lights_from_json(&json["lights"]);
    }
}
