//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// json parsing
//

use std::fs;

use crate::vectors;
use serde_json::Value;
use vectors::Vector;
use super::Renderer;
use super::camera::{Lens, Camera};
use super::primitives::{Sphere, Plane, Cylinder, Cone, Object, Triangle};
use super::primitives::mesh::Mesh;
use super::lights::{Point, Ambient, Light, Lights, Directional};
use super::renderer_common::{Transform, Color, Texture, Image};

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
            scale: json["scale"].as_f64().unwrap_or(1.0),
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
            reflection_samples: json["reflection_samples"].as_i64().unwrap_or(16),
            threads: json["threads"].as_u64().unwrap_or(8),
            progression: json["progression"].as_bool().unwrap_or(false),
            super_sampling: json["super_sampling"].as_u64().unwrap_or(1),
            super_sampling_precision: json["super_sampling_precision"].as_u64().unwrap_or(10),
            image_buffer_size: json["image_buffer_size"].as_u64().unwrap_or(1),
            reflecion_samples: json["reflection_samples"].as_f64().unwrap_or(16.0),
        };
        camera.calculate_lens_distance();
        camera.calculate_lens_size();

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

    pub fn get_image_from_json(&self, json: &Value) -> Image {
        let filename = json["file"].as_str().unwrap_or("missing_texture.ppm").to_string();
        let data = fs::read_to_string(filename).unwrap_or(fs::read_to_string("missing_texture.ppm").expect("missing missing texture texture\n"));

        print!("{}\n", data);
        Image {
            height: 0,
            width: 0,
            vector: Vec::new(),
        }
    }

    pub fn get_texture_from_json(&self, json: &Value) -> Texture {
        Texture {
            texture_type: json["texture_type"].as_u64().unwrap_or(0),
            color: if json["color"].is_object(){self.get_color_from_json(&json["color"])} else {Color::default()},
            secondary_color: if json["secondary_color"].is_object(){self.get_color_from_json(&json["secondary_color"])} else {Color::default()},
            image: if json["image"].is_object() {self.get_image_from_json(&json["image"])} else {Image::default()},
            mod1: json["mod1"].as_f64().unwrap_or(2.0),
            mod2: json["mod2"].as_f64().unwrap_or(2.0),
            diffuse: json["diffuse"].as_f64().unwrap_or(0.7),
            ambient: json["ambient"].as_f64().unwrap_or(0.1),
            specular: json["specular"].as_f64().unwrap_or(0.4),
            metalness: json["metalness"].as_f64().unwrap_or(0.1),
            shininess: json["shininess"].as_f64().unwrap_or(4.0),
            roughness: json["roughness"].as_f64().unwrap_or(0.25),
            sampling_ponderation: json["sampling_ponderation"].as_f64().unwrap_or(1.0),
            alpha: json["alpha"].as_f64().unwrap_or(1.0),
        }
    }

    pub fn get_sphere_from_json(&self, json: &Value) -> Box::<Sphere> {
        let mut sphere = Sphere {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
            radius: json["radius"].as_f64().unwrap_or(1.0),
            radius_applied: 0.0,
        };
        sphere.apply_transform();
        Box::new(sphere)
    }

    pub fn get_plane_from_json(&self, json: &Value) -> Box::<Plane> {
        let mut plane = Plane {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
            normal: if json["normal"].is_object(){self.get_vector_from_json(&json["normal"])} else {Vector {x: 0.0, y: 0.0, z: 1.0}},
            normal_applied: Vector { x: 0.0, y: 0.0, z: 0.0, },
        };
        plane.apply_transform();
        Box::new(plane)
    }

    pub fn get_cylinder_from_json(&self, json: &Value) -> Box::<Cylinder> {
        let mut cylinder = Cylinder {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
            height: json["height"].as_f64().unwrap_or(2.0),
            radius: json["radius"].as_f64().unwrap_or(1.0),
            axis: Vector { x: 0.0, y: 0.0, z: 0.0, },
            top: Vector { x: 0.0, y: 0.0, z: 0.0, },
            base: Vector { x: 0.0, y: 0.0, z: 0.0, },
            radius_applied: 0.0,
            height_applied: 0.0,
        };
        cylinder.apply_transform();
        Box::new(cylinder)
    }

    pub fn get_cone_from_json(&self, json: &Value) -> Box::<Cone> {
        let mut cone = Cone {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
            height: json["height"].as_f64().unwrap_or(3.0),
            radius: json["radius"].as_f64().unwrap_or(1.0),
            axis: Vector { x: 0.0, y: 0.0, z: 0.0, },
            top: Vector { x: 0.0, y: 0.0, z: 0.0, },
            base: Vector { x: 0.0, y: 0.0, z: 0.0, },
            radius_applied: 0.0,
            height_applied: 0.0,
        };
        cone.apply_transform();
        Box::new(cone)
    }

    pub fn get_triangle_from_json(&self, json: &Value) -> Box<Triangle> {
        let mut triangle = Triangle {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
            point_a: if json["point_a"].is_object() {self.get_vector_from_json(&json["point_a"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            point_b: if json["point_b"].is_object() {self.get_vector_from_json(&json["point_b"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            point_c: if json["point_c"].is_object() {self.get_vector_from_json(&json["point_c"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
            point_a_applied: Vector { x: 0.0, y: 0.0, z: 0.0, },
            point_b_applied: Vector { x: 0.0, y: 0.0, z: 0.0, },
            point_c_applied: Vector { x: 0.0, y: 0.0, z: 0.0, },
            normal: Vector { x: 0.0, y: 0.0, z: 0.0, },
        };
        triangle.apply_transform();
        Box::new(triangle)
    }

    pub fn get_mesh_from_json(&self, json: &Value) -> Box<Mesh> {
        let mut mesh = Mesh {
            transform: if json["transform"].is_object() {self.get_transform_from_json(&json["transform"])} else {Transform::default()},
            texture: if json["texture"].is_object() {self.get_texture_from_json(&json["texture"])} else {Texture::default()},
            triangles: Vec::new(),
        };
        if json["file"].is_string() {
            let filename = json["file"].as_str().unwrap();
            mesh.parse_obj(filename);
        }
        mesh.apply_transform();
        Box::new(mesh)
    }

    pub fn get_object_from_json(&self, json: &Value) -> Option<Box::<dyn Object + Send + Sync>> {
        if json["type"].is_string() {
            return match json["type"].as_str().unwrap() {
                "sphere" => Some(self.get_sphere_from_json(json)),
                "plane" => Some(self.get_plane_from_json(json)),
                "cylinder" => Some(self.get_cylinder_from_json(json)),
                "cone" => Some(self.get_cone_from_json(json)),
                "triangle" => Some(self.get_triangle_from_json(json)),
                "mesh" => Some(self.get_mesh_from_json(json)),
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

    pub fn get_directional_from_json(&self, json: &Value) -> Box::<Directional> {
        let mut result = Box::new(
            Directional {
                transform: Transform {
                    pos: Vector {x: 0.0, y: 0.0, z: 1.0},
                    rotation: if json["transform"]["rotation"].is_object() {self.get_vector_from_json(&json["transform"]["rotation"])} else {Vector {x: 0.0, y: 0.0, z: 0.0}},
                    scale: json["transform"]["scale"].as_f64().unwrap_or(1.0),
                },
                color: if json["color"].is_object() {self.get_color_from_json(&json["color"])} else {Color::default()},
                strength: json["strength"].as_f64().unwrap_or(80.0),
                visible: json["visible"].as_bool().unwrap_or(false),
            }
        );
        result.transform.pos.rotate(result.transform.rotation.x, result.transform.rotation.y, result.transform.rotation.z);
        result
    }

    pub fn get_object_lights_from_json(&self, json: &Value) -> Vec::<Box::<dyn Light + Send + Sync>> {
        let mut lights: Vec::<Box::<dyn Light + Send + Sync>> = Vec::new();

        if json["point"].is_array(){
            for point in json["point"].as_array().unwrap().iter() {
                lights.push(self.get_point_from_json(point))
            }
        }
        if json["directional"].is_array(){
            for point in json["directional"].as_array().unwrap().iter() {
                lights.push(self.get_directional_from_json(point))
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
