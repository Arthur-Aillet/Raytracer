//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer
//

use vectors::VectorF;
use crate::vectors;
use crate::vectors::Point;

#[derive(Debug, Clone)]
pub struct Transform {
    pos: vectors::Point,
    rotation : vectors::Point,
    scale : vectors::Point,
}

impl Transform {
    pub fn new (x_pos: f64, y_pos: f64, z_pos: f64, x_rot: f64, y_rot: f64, z_rot: f64, x_sca: f64, y_sca: f64,  z_sca: f64) -> Self {
        Transform { pos: Point {x:x_pos, y:y_pos, z:z_pos}, rotation: Point{x:x_rot, y:y_rot, z:z_rot}, scale: Point{x:z_sca, y:z_sca, z:z_sca}}
    }
}

#[derive(Debug)]
pub struct Renderer {
    camera: Camera,
}

#[derive(Debug)]
struct Lens {
    height : i64,
    width : i64,
    distance : f64,
    vector_to_first_pixel : VectorF,
}

#[derive(Debug)]
pub struct Camera {
    transform : Transform,
    lens : Lens,
    fov : i16,
}

impl Camera {
    fn new() -> Self {
        let mut result = Camera {
            transform : Transform::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            fov : 70,
            lens : Lens {
                width : 1920,
                height : 1080,
                distance : 0.0,
                vector_to_first_pixel : VectorF { origin: Point { x:0.0, y:0.0, z:0.0 }, direction: Point { x:0.0, y:0.0, z:0.0 } },
            }
        };
        result.calculate_lens_distance();
        result.lens.vector_to_first_pixel.direction.x = -result.lens.width as f64 / 2.0;
        result.lens.vector_to_first_pixel.direction.y = result.lens.height as f64 / 2.0;
        result.lens.vector_to_first_pixel.direction.z = result.lens.distance + result.lens.width as f64 / 2.0;
        result
    }

    fn get_pixel_vector(&self, x:i64, y:i64) -> VectorF {
        let mut vectors = vectors::VectorF {
            origin: self.transform.pos.clone(),
            direction: vectors::Point {
                x: self.lens.vector_to_first_pixel.direction.x + x as f64,
                y: self.lens.vector_to_first_pixel.direction.y + y as f64,
                z: self.lens.vector_to_first_pixel.direction.z,
            },
        };
        vectors.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        vectors
    }

    fn calculate_lens_distance(&mut self) {
        self.lens.distance = (self.lens.height as f64 / 2.0) / (self.fov as f64).tan();
    }
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            camera: Camera::new()
        }
    }

    pub fn render() -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        pixels
    }
}
