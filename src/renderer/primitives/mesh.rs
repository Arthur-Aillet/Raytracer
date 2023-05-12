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
    pub fn parse_face(&mut self, line :String, verteces : &Vec<Vector>) -> (Option<Triangle>, Option<Triangle>){
        let points: Vec<&str> = line.split_ascii_whitespace()
                                    .filter(|&x| !x.is_empty())
                                    .skip(1)
                                    .collect();
        let vertices_available = verteces.len();
        let len = points.len();
        if len < 3 || len > 4 {
            return (None, None);
        }

        let mut points_res = [Vector { x: 0.0, y: 0.0, z: 0.0, }; 4];
        let mut count = 0;

        for point in points {
            let mut iter = &mut point.split('/');

            for (i, item) in iter.take(3).enumerate() {
                if i == 1 && item.is_empty() {
                    continue;
                }
                match item.parse::<usize>() {
                    Ok(num) => if i == 0 {
                        if (num - 1) > vertices_available {
                            return (None, None)
                        }
                        points_res[count] = verteces[num - 1];
                    }
                    Err(_) => return (None, None),
                }
            }
            if iter.next().is_some() {
                return (None, None);
            }
            count += 1;
        }
        let fst_triangle: Triangle = Triangle {
            transform: self.transform,
            texture: self.texture.clone(),
            point_a: points_res[0],
            point_b: points_res[1],
            point_c: points_res[2],
        };
        if len == 3 {
            return (Some(fst_triangle), None);
        }
        let snd_triangle: Triangle = Triangle {
            transform: self.transform,
            texture: self.texture.clone(),
            point_a: points_res[2],
            point_b: points_res[3],
            point_c: points_res[0],
        };
        return (Some(fst_triangle), Some(snd_triangle));
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
                        let face_parsed = self.parse_face(line, &vertexes);
                        if let Some(face_fst) = face_parsed.0 {
                            self.triangles.push(face_fst);
                            if let Some(face_snd) = face_parsed.1 {
                                self.triangles.push(face_snd);
                            }
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