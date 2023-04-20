//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// mod
//

use std::fs;
use crate::vectors;
use serde_json::Value;
use vectors::Segment;
use vectors::Vector;
use vectors::resolve_quadratic_equation;


#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vector,
    pub rotation: Vector,
    pub scale: Vector,
}

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
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    pub texture_type: u64,
    pub color: Color,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub shininess: f64,
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
    pub origin: Vector,
    pub vector: Vector,
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

pub trait Object {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment>;
    fn obj_type(&self) -> String;
    fn get_transform(&self) -> Transform;
    fn set_transform(&mut self, new: Transform);
    fn get_texture(&self) -> Texture;
    fn set_texture(&mut self, new: Texture);
    fn get_radius(&self) -> f64;
    fn set_radius(&mut self, new: f64);
    fn get_height(&self) -> f64;
    fn set_height(&mut self, new: f64);
    fn get_origin(&self) -> Vector;
    fn set_origin(&mut self, new: Vector);
    fn get_vector(&self) -> Vector;
    fn set_vector(&mut self, new: Vector);
    fn slanted_height(&self) -> f64;
    fn diameter(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn surface(&self) -> f64;
    fn lateral_surface(&self) -> f64;
    fn base_surface(&self) -> f64;
    fn volume(&self) -> f64;
}

impl Object for Sphere {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {
        let diff = camera - self.transform.pos;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - self.radius.powi(2));

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|a, b| a.partial_cmp(b).unwrap());
        //filter neg
        if smallest_result == None {
            None
        } else {
            Some ( Segment {
                origin : self.transform.pos,
                end: Vector {
                    x: camera.x + ray.x * smallest_result.unwrap(),
                    y: camera.y + ray.y * smallest_result.unwrap(),
                    z: camera.z + ray.z * smallest_result.unwrap(),
                }
            })
        }
    }
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
    fn get_origin(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Vector) {}
    fn get_vector(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Vector) {}
    fn slanted_height(&self) -> f64 {1.0}
    fn lateral_surface(&self) -> f64 {self.surface()}
    fn base_surface(&self) -> f64 {self.surface()}
}

impl Object for Plane {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {return None;}
    fn obj_type(&self) -> String {format!("plane")}
    fn get_transform(&self) -> Transform {self.transform}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn get_origin(&self) -> Vector {self.origin}
    fn set_origin(&mut self, new: Vector) {self.origin = new}
    fn get_vector(&self) -> Vector {self.vector}
    fn set_vector(&mut self, new: Vector) {self.vector = new}

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
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {return None;}
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

    fn get_origin(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Vector) {}
    fn get_vector(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Vector) {}
    fn slanted_height(&self) -> f64 {1.0}
}

impl Object for Cone {
    fn intersection(&self, ray: Vector, camera: Vector) -> Option<Segment> {return None;}
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

    fn get_origin(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_origin(&mut self, _new: Vector) {}
    fn get_vector(&self) -> Vector {
        let a = Vector{x: 0.0, y: 0.0, z: 0.0};
        a
    }
    fn set_vector(&mut self, _new: Vector) {}
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
        self.camera.calculate_lens_distance();
        self.primitives = self.get_objects_from_json(&json["primitives"]);
        self.lights = self.get_lights_from_json(&json["lights"]);
    }

    pub fn render(&mut self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {
                let camera_to_pixel = self.camera.get_pixel_vector(j, i);
                // for object in self.primitives.iter_mut() {
                    let intersect = self.primitives[0].intersection(camera_to_pixel, self.camera.transform.pos);
                    if intersect != None {
                        print!("{}\n", self.primitives[0].obj_type());
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
