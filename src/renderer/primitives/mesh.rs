//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// mesh
//

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use crate::renderer::primitives::{Intersection, Object, Triangle};
use crate::renderer::renderer_common::{Texture, Transform};
use crate::vectors::Vector;
use serde::{Deserialize, Serialize};
use erased_serde::serialize_trait_object;

#[derive(Deserialize, Serialize)]
pub struct Mesh {
    pub transform: Transform,
    pub texture: Texture,
    pub triangles: Vec<Triangle>,
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
        for coord in [&mut new_vertex.x, &mut new_vertex.z, &mut new_vertex.y].iter_mut() {
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
                    else if line.starts_with('#') {
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