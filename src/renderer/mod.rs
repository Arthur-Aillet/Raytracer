//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer
//

pub mod primitives;
use crate::vectors;
use vectors::Point;
use vectors::VectorF;
use crate::renderer::primitives::{Object, Sphere, Light};

#[derive(Debug, Clone)]
pub struct Transform {
    pos: vectors::Point,
    rotation : vectors::Point,
    scale : vectors::Point,
}

impl Transform {
    pub fn new(
        x_pos: f64,
        y_pos: f64,
        z_pos: f64,
        x_rot: f64,
        y_rot: f64,
        z_rot: f64,
        x_sca: f64,
        y_sca: f64,
        z_sca: f64,
    ) -> Self {
        Transform {
            pos: Point {
                x: x_pos,
                y: y_pos,
                z: z_pos,
            },
            rotation: Point {
                x: x_rot,
                y: y_rot,
                z: z_rot,
            },
            scale: Point {
                x: z_sca,
                y: z_sca,
                z: z_sca,
            },
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    camera: Camera,
    object: Sphere,
    light: Light,
}

#[derive(Debug)]
struct Lens {
    height: i64,
    width: i64,
    distance: f64,
    vector_to_first_pixel: VectorF,
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
            transform: Transform::new(0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0, 0.0),
            fov: 70,
            lens: Lens {
                width: 1920,
                height: 1080,
                distance: 0.0,
                vector_to_first_pixel: VectorF {
                    origin: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    direction: Point {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
            },
        };
        result.calculate_lens_distance();
        let mut vector_director = VectorF::new(0.0, result.lens.distance, 0.0);
        result.lens.vector_to_first_pixel = VectorF::new(result.transform.pos.x, result.transform.pos.y, result.transform.pos.z);
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + VectorF::new(0.0, 0.0, 1.0) * (result.lens.height as f64 / 2.0);
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + vector_director;
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + VectorF::new(-1.0, 0.0, 0.0) * (result.lens.width as f64 / 2.0);
        println!("{:?}", result.lens.vector_to_first_pixel);
        result
    }

    fn get_pixel_vector(&self, x: i64, y: i64) -> VectorF {
        let mut pixel_vector = self.lens.vector_to_first_pixel.clone();

        pixel_vector = pixel_vector + VectorF::new(0.0, 0.0, -1.0) * x as f64;
        pixel_vector = pixel_vector + VectorF::new(1.0, 0.0, 0.0) * y as f64;
        pixel_vector.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        pixel_vector
    }

    fn calculate_lens_distance(&mut self) {
        self.lens.distance = (self.lens.height as f64 / 2.0) / (self.fov as f64).tan();
    }
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            camera: Camera::new(),
            object: Sphere {
                origin: Point {x:0.0, y:40.0, z:0.0},
                radius: 15.0
            },
            light: Light {
                origin: Point {x:0.0, y:-10.0, z:20.0},
            }
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {

                let intersect =  self.object.intersection(self.camera.get_pixel_vector(i, j));
                if intersect != None {
                    let mut color: f64 = 255.0 * 0.1;

                    let light_vector = Point {
                        x: self.light.origin.x - intersect.unwrap().direction.x,
                        y: self.light.origin.y - intersect.unwrap().direction.y,
                        z: self.light.origin.z - intersect.unwrap().direction.z,
                    };
                    let normal_vector = Point {
                        x: intersect.unwrap().direction.x - intersect.unwrap().origin.x,
                        y: intersect.unwrap().direction.y - intersect.unwrap().origin.y,
                        z: intersect.unwrap().direction.z - intersect.unwrap().origin.z,
                    };

                    color += (light_vector.dot_product(&normal_vector)).max(0.0) * 0.7;
                    pixels.extend(&[color as u8, color as u8, color as u8]);
                } else {
                    pixels.extend(&[0x00, 0x00, 0x00]);
                }
            }
        }
        pixels
    }
}
