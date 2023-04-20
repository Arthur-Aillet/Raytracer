//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// mod
//

pub mod primitives;

use std::fs;
use crate::vectors;
use primitives::{Transform, Color, Texture, Object, Sphere, Plane, Cylinder, Cone};
use serde_json::Value;
use vectors::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Lens {
    pub height: i64,
    pub width: i64,
    pub distance: f64,
    pub vector_to_first_pixel: Vector,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub transform: Transform,
    pub lens: Lens,
    pub fov : i64,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
}

impl Camera {

    fn get_pixel_vector(&self, x: i64, y: i64) -> Vector {
        let mut pixel_vector = self.lens.vector_to_first_pixel.clone();

        pixel_vector = pixel_vector + Vector {x:1.0, y:0.0, z:0.0} * x as f64;
        pixel_vector = pixel_vector + Vector {x:0.0, y:0.0, z:-1.0} * y as f64;
        pixel_vector.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        pixel_vector.normalize()
    }
// Point { x: -960.0, y: 441.91302184715596, z: 540.0 } }
    fn calculate_lens_distance(&mut self) {
        self.lens.distance = (self.lens.height as f64 / 2.0) / (self.fov as f64).to_radians().tan();
    }

    pub fn calculate_tone_mapping(val: f64) -> f64{
        let a = 2.51;
        let b = 0.03;
        let c = 2.43;
        let d = 0.59;
        let e = 0.14;
        ((val * (a * val + b))/(val * (c * val + d) + e)).clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Copy)]
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
                    pos: Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    },
                    rotation: Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    },
                    scale: Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0
                    }
                },
                lens: Lens {
                    height: 0,
                    width: 0,
                    distance: 0.0,
                    vector_to_first_pixel: Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                fov: 80,
                diffuse: 10.0,
                ambient: 10.0,
                specular: 10.0,
            },
            primitives: Vec::new(),
            lights: Lights {
                lights: Vec::new(),
                ambiant: Vec::new(),
            },
        };
        renderer
    }

    fn get_vector_from_json(&self, json: &Value) -> Vector{
        let vector = Vector {
            x: json["x"].as_f64().unwrap(),
            y: json["y"].as_f64().unwrap(),
            z: json["z"].as_f64().unwrap()
        };
        vector
    }

    fn get_transform_from_json(&self, json: &Value) -> Transform {
        let transform = Transform {
            pos: self.get_vector_from_json(&json["pos"]),
            rotation: self.get_vector_from_json(&json["rotation"]),
            scale: self.get_vector_from_json(&json["scale"])
        };
        transform
    }

    fn get_lens_from_json(&self, json: &Value) -> Lens {
        let lens = Lens {
            height: json["height"].as_i64().unwrap(),
            width: json["width"].as_i64().unwrap(),
            distance: json["distance"].as_f64().unwrap(),
            vector_to_first_pixel: self.get_vector_from_json(&json["vector_to_first_pixel"]),
        };
        lens
    }

    fn get_camera_from_json(&self, json: &Value) -> Camera {
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

    fn get_color_from_json(&self, json: &Value) -> Color {
        let color = Color {
            r: (json["r"].as_f64().unwrap() % 255.0),
            g: (json["g"].as_f64().unwrap() % 255.0),
            b: (json["b"].as_f64().unwrap() % 255.0),
        };
        color
    }

    fn get_texture_from_json(&self, json: &Value) -> Texture {
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
            origin: self.get_vector_from_json(&json["origin"]),
            vector: self.get_vector_from_json(&json["vector"]),
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

    pub fn render(&mut self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {
                let camera_to_pixel = self.camera.get_pixel_vector(j, i);
                // for object in self.primitives.iter_mut() {
                    let intersect = self.primitives[0]  .intersection(camera_to_pixel, self.camera.transform.pos);
                    if intersect != None {
                        // let light_vector = (self.light.origin - intersect.unwrap().end).normalize(); <- je sais pas comment mettre ca dans une boucle pour toutes les lumieres
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
        // }
        pixels
    }
}
