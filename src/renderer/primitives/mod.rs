//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

use crate::vectors;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use vectors::Vector;
use vectors::resolve_quadratic_equation;
use crate::renderer::lights::Light;
use super::renderer_common::{Transform, Texture};
use serde::{Deserialize, Serialize};
use erased_serde::serialize_trait_object;

pub struct Intersection<'a> {
    pub intersection_point: Vector,
    pub normal: Vector,
    pub object: Option<&'a dyn Object>,
    pub light: Option<&'a dyn Light>,
}

#[derive(Deserialize, Serialize)]
pub struct Sphere {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Plane {
    pub transform: Transform,
    pub texture: Texture,
    pub normal: Vector,
}

#[derive(Deserialize, Serialize)]
pub struct Cylinder {
    pub transform: Transform,
    pub texture: Texture,
    pub height: f64,
    pub radius: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Cone {
    pub transform: Transform,
    pub texture: Texture,
    pub radius: f64,
    pub height: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Triangle {
    pub transform : Transform,
    pub texture: Texture,
    pub point_a: Vector,
    pub point_b: Vector,
    pub point_c: Vector
}

#[derive(Deserialize, Serialize)]
pub struct Mesh {
    pub transform: Transform,
    pub texture: Texture,
    pub triangles: Vec<Triangle>,
}

pub trait Object: erased_serde::Serialize {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
    fn surface_position(&self, position: Vector) -> Vector;
    fn get_transform(&self) -> Transform;
    fn move_obj(&mut self, offset: Transform);
    fn set_transform(&mut self, new: Transform);
    fn get_texture(&self) -> Texture;
    fn set_texture(&mut self, new: Texture);
    fn set_radius(&mut self, new: f64);
    fn set_height(&mut self, new: f64);
    fn set_normal(&mut self, new: Vector);
    fn set_triangles(&mut self, new: String);
    fn set_points(&mut self, new_a: Vector, new_b: Vector, new_c: Vector);
}

impl Object for Sphere {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(ray.dot_product(ray), // could be 1 if normalized
                                                2.0 * (ray.dot_product(diff)),
                                                (diff.dot_product(diff)) - (self.radius * self.transform.scale.x).powi(2));

        let smallest_result: Option<&f64> = result.iter().filter(|number| **number > 0.0).min_by(|a, b| a.partial_cmp(b).unwrap());

        if smallest_result == None {
            None
        } else {
            let point = Vector {
                x: origin.x + ray.x * smallest_result.unwrap(),
                y: origin.y + ray.y * smallest_result.unwrap(),
                z: origin.z + ray.z * smallest_result.unwrap(),
            };
            Some ( Intersection {
                normal: point - self.transform.pos,
                intersection_point: point,
                object: Some(self),
                light: None
            })
        }
    }

    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: 2.0 * (1.0 - (rotated_position.x.atan2(rotated_position.y)/ (2.0 * std::f64::consts::PI) + 0.5)),
            y: 1.0 - (rotated_position.z / (rotated_position.x.powi(2) + rotated_position.y.powi(2) + rotated_position.z.powi(2)).sqrt()).acos() / std::f64::consts::PI,
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, _new: Vector) {}
    fn set_triangles(&mut self, _new: String) {}
    fn set_points(&mut self, _new_a: Vector, _new_b: Vector, _new_c: Vector) {}
}

impl Object for Plane {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut normal = self.normal.normalize();
        normal.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        let denom = ray.normalize().dot_product(normal);
        if denom == 0.0 {
            return None
        }
        let progress = (self.transform.pos - origin).dot_product(normal) / denom;
        if progress < 0.0 {
            return None
        }
        Some ( Intersection {
            intersection_point: Vector{
                x: origin.x + ray.x * progress,
                y: origin.y + ray.y * progress,
                z: origin.z + ray.z * progress
            },
            normal,
            object: Some(self),
            light: None,
        })
    }
    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: if position.x < 0.0 {position.x % 1.0 + 1.0} else {position.x % 1.0},
            y: if position.y < 0.0 {position.y % 1.0 + 1.0} else {position.y % 1.0},
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, _new: f64) {}

    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, new: Vector) {self.normal = new}
    fn set_triangles(&mut self, _new: String) {}
    fn set_points(&mut self, _new_a: Vector, _new_b: Vector, _new_c: Vector) {}
}

impl Object for Cylinder {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {return None;}
    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        Vector {
            x: 1.0 - (rotated_position.x.atan2(rotated_position.y) / (2.0 * std::f64::consts::PI) + 0.5),
            y: rotated_position.z % 1.0,
            z: 0.0
        }
    }
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, new: f64) {self.height = new}
    fn set_normal(&mut self, _new: Vector) {}
    fn set_triangles(&mut self, new: String) {}
    fn set_points(&mut self, new_a: Vector, new_b: Vector, new_c: Vector) {}
}

impl Object for Cone {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {return None;}
    fn surface_position(&self, position: Vector) -> Vector {Vector { x: 0.5, y: 0.5, z: 0.0}}
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, new: f64) {self.radius = new}

    fn set_height(&mut self, new: f64) {self.height = new}
    fn set_normal(&mut self, _new: Vector) {}
    fn set_triangles(&mut self, new: String) {}
    fn set_points(&mut self, new_a: Vector, new_b: Vector, new_c: Vector) {}
}

impl Object for Triangle {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut point_a = self.point_a.clone();
        point_a.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        point_a = point_a + self.transform.pos;
        let mut point_b = self.point_b.clone();
        point_b.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        point_b = point_b + self.transform.pos;
        let mut point_c = self.point_c.clone();
        point_c.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        point_c = point_c + self.transform.pos;

        let mut normal = (point_b - point_a).cross_product(point_c - point_a).normalize();

        let denom = ray.normalize().dot_product(normal);
        if denom == 0.0 {
            return None
        }
        let progress = (self.transform.pos - origin).dot_product(normal) / denom;
        if progress < 0.0 {
            return None
        }
        let intersection_point = Vector{
            x: origin.x + ray.x * progress,
            y: origin.y + ray.y * progress,
            z: origin.z + ray.z * progress
        };

        let cross = (point_b - point_a).cross_product(intersection_point - point_a);
        if normal.dot_product(cross) < 0.0 {
            return None;
        }

        let cross = (point_c - point_b).cross_product(intersection_point - point_b);
        if normal.dot_product(cross) < 0.0 {
            return None;
        }

        let cross = (point_a - point_c).cross_product(intersection_point - point_c);
        if normal.dot_product(cross) < 0.0 {
            return None;
        }

        if normal.dot_product(origin - intersection_point) < 0.0 {
            normal = normal * -1.0;
        }

        Some ( Intersection {
            intersection_point,
            normal,
            object: Some(self),
            light: None,
        })
    }
    fn surface_position(&self, position: Vector) -> Vector {Vector { x: 0.5, y: 0.5, z: 0.0}}
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, _new: f64) {}

    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, _new: Vector) {}
    fn set_triangles(&mut self, _new: String) {}
    fn set_points(&mut self, new_a: Vector, new_b: Vector, new_c: Vector) {
        self.point_a = new_a;
        self.point_b = new_b;
        self.point_c = new_c;
    }
}

impl Mesh {
    pub fn parse_face(&mut self, line :String, verteces : &Vec<Vector>) -> Option<Triangle>{
        let mut new_triangle: Triangle = Triangle {
            transform: self.transform,
            texture: self.texture.clone(),
            point_a: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            point_b: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            point_c: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let mut iter = line.split_ascii_whitespace().filter(|&x| !x.is_empty());

        iter.next();
        for coord in [&mut new_triangle.point_a, &mut new_triangle.point_b, &mut new_triangle.point_c].iter_mut() {
            if let Some(point) = iter.next() {
                let mut split = point.split('/');

                let str = split.next();
                if str.is_none() {
                    return None;
                }
                if let Ok(id) = str.unwrap().parse::<usize>() {
                    if verteces.len() < id {
                        return None;
                    }
                    **coord = verteces[id - 1];
                } else {
                    return None;
                };
            } else {
                return None;
            }
        }
        Some(new_triangle)
    }

    pub fn parse_vertex(&mut self, line: String) -> Option<Vector> {
        let mut new_vertex: Vector = Vector { x: 0.0, y: 0.0, z: 0.0, };
        let mut iter = line.split_ascii_whitespace().filter(|&x| !x.is_empty());

        iter.next();
        for coord in [&mut new_vertex.x, &mut new_vertex.y, &mut new_vertex.z].iter_mut() {
            if let Some(point) = iter.next() {
                if let Ok(point) = (point).parse::<f64>() {
                    **coord = point;
                } else {
                    return None;
                };
            } else {
                return None;
            }
        }
        println!("{new_vertex:?}");
        Some(new_vertex)
    }
    pub fn parse_obj(&mut self, file_name: &str) {
        let file = OpenOptions::new().read(true).open(file_name);

        if let Ok(obj) = file {

            let mut vertexes: Vec<Vector> = Vec::new();

            for option_line in BufReader::new(obj).lines() {
                if let Ok(line) = option_line {
                    if line.chars().all(|x| x.is_ascii_whitespace()) {
                        continue;
                    }
                    else if line.starts_with("# ") {
                        continue;
                    }
                    else if line.starts_with("o ") {
                        continue;
                    }
                    else if line.starts_with("vn ") {
                        continue;
                    }
                    else if line.starts_with("vt ") {
                        continue;
                    }
                    else if line.starts_with("v ") {
                        if let Some(vertex) = self.parse_vertex(line) {
                            vertexes.push(vertex);
                        } else {
                            println!("Invalid vertexes in \"{}\" !", file_name);
                            return;
                        }
                    }
                    else if line.starts_with("s ") {
                        continue;
                    }
                    else if line.starts_with("f ") {
                        if let Some(face) = self.parse_face(line, &vertexes) {
                            self.triangles.push(face);
                        } else {
                            println!("Invalid face in \"{}\" !", file_name);
                            return;
                        }
                    }
                    else if line.starts_with("mtllib ") {
                        continue;
                    } else {
                        println!("Invalid \"{}\" mesh file!", file_name);
                        return;
                    }
                }
            }
        } else {
            println!("Cant open \"{}\" mesh file!", file_name);
        }
    }
}

impl Object for Mesh {
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut first_intersection: Option<Intersection> = None;

        for face in &self.triangles {
            if let Some(intersection) = face.intersection(ray, origin) {
                if let Some(first) = &first_intersection {
                    if (first.intersection_point - origin).len() > (intersection.intersection_point - origin).len() {
                        first_intersection = Some(intersection);
                    }
                } else {
                    first_intersection = Some(intersection);
                }
            }
        }
        first_intersection
    }
    fn surface_position(&self, position: Vector) -> Vector {Vector { x: 0.5, y: 0.5, z: 0.0}}
    fn get_transform(&self) -> Transform {self.transform}
    fn move_obj(&mut self, offset: Transform) {self.transform = self.transform + offset;}
    fn set_transform(&mut self, new: Transform) {self.transform = new}
    fn get_texture(&self) -> Texture {self.texture.clone()}
    fn set_texture(&mut self, new: Texture) {self.texture = new}
    fn set_radius(&mut self, _new: f64) {}

    fn set_height(&mut self, _new: f64) {}
    fn set_normal(&mut self, _new: Vector) {}
    fn set_triangles(&mut self, new: String) {self.triangles = Vec::new()}
    fn set_points(&mut self, _new_a: Vector, _new_b: Vector, _new_c: Vector) {}
}

serialize_trait_object!(Object);
