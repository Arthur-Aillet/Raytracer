//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer
//

pub mod primitives;
use crate::vectors;
use vectors::Vector;
use vectors::Segment;
use crate::renderer::primitives::{Object, Sphere, Light};

#[derive(Debug, Clone)]
pub struct Transform {
    pos: vectors::Vector,
    rotation : vectors::Vector,
    scale : vectors::Vector,
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
            pos: Vector {
                x: x_pos,
                y: y_pos,
                z: z_pos,
            },
            rotation: Vector {
                x: x_rot,
                y: y_rot,
                z: z_rot,
            },
            scale: Vector {
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
    vector_to_first_pixel: Vector,
}

#[derive(Debug)]
pub struct Camera {
    transform : Transform,
    lens : Lens,
    fov : i16,
    diffuse: f64,
    ambient: f64,
    specular: f64,
}

impl Camera {
    fn new() -> Self {
        let mut result = Camera {
            transform: Transform::new(0.0, 0.0, 0.0, 0.0,1.0, 18.0, 0.0, 0.0, 0.0),
            fov: 70,
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.7,
            lens: Lens {
                width: 1920,
                height: 1080,
                distance: 0.0,
                vector_to_first_pixel: Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                },
            },
        };
        result.calculate_lens_distance();
        let vector_director = Vector {x:0.0, y:result.lens.distance, z:0.0};
        result.lens.vector_to_first_pixel = Vector {x:result.transform.pos.x, y:result.transform.pos.y, z:result.transform.pos.z};
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + Vector {x:0.0, y:0.0, z:1.0} * (result.lens.height as f64 / 2.0);
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + vector_director;
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + Vector {x:-1.0, y:0.0, z:0.0} * (result.lens.width as f64 / 2.0);
        println!("{:?}", result.lens.vector_to_first_pixel);
        result
    }

    fn get_pixel_vector(&self, x: i64, y: i64) -> Vector {
        let mut pixel_vector = self.lens.vector_to_first_pixel.clone();

        pixel_vector = pixel_vector + Vector {x:0.0, y:0.0, z:-1.0} * x as f64;
        pixel_vector = pixel_vector + Vector {x:1.0, y:0.0, z:0.0} * y as f64;
        pixel_vector.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        pixel_vector
    }
// Point { x: -960.0, y: 441.91302184715596, z: 540.0 } }
    fn calculate_lens_distance(&mut self) {
        self.lens.distance = (self.lens.height as f64 / 2.0) / (self.fov as f64).tan();
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

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            camera: Camera::new(),
            object: Sphere {
                origin: Vector {x:0.0, y:4.0, z:0.0},
                radius: 1.5,
                ambient: 0.1,
                diffuse: 0.7,
                specular: 0.8,
                shininess: 10.0,
                r: 0,
                g: 255,
                b: 255,
            },
            light: Light {
                origin: Vector {x:0.0, y:-3.0, z:0.0},
                intensity: 1000.0,
            }
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {
                let camera_to_pixel = self.camera.get_pixel_vector(i, j);
                let intersect = self.object.intersection(camera_to_pixel, self.camera.transform.pos);
                if intersect != None {
                    let mut light_vector = Vector {
                        x: self.light.origin.x - intersect.unwrap().direction.x,
                        y: self.light.origin.y - intersect.unwrap().direction.y,
                        z: self.light.origin.z - intersect.unwrap().direction.z,
                    };
                    let mut normal_vector = Vector {
                        x: intersect.unwrap().direction.x - intersect.unwrap().origin.x,
                        y: intersect.unwrap().direction.y - intersect.unwrap().origin.y,
                        z: intersect.unwrap().direction.z - intersect.unwrap().origin.z,
                    };

                    light_vector.normalize();
                    normal_vector.normalize();

                    let mut light_intensity: f64 = self.camera.diffuse * self.object.ambient;
                    light_intensity += (light_vector.dot_product(normal_vector)).max(0.0) * self.camera.diffuse * self.object.diffuse;

                    let mut reflected: Vector = light_vector.clone();
                    reflected.reflect(normal_vector);
                    reflected.normalize();
                    let mut view = camera_to_pixel.clone() * -1.0;
                    view.normalize();

                    let specular = 0.6 * 0.8 * reflected.dot_product(view).max(0.0).powf(8.0);

                    pixels.extend(&[
                        (light_intensity * self.object.r as f64 + specular * 255.0) as u8,
                        (light_intensity * self.object.g as f64 + specular * 255.0) as u8,
                        (light_intensity * self.object.b as f64 + specular * 255.0) as u8
                    ]);
                } else {
                    pixels.extend(&[0x00, 0x00, 0x00]);
                }
            }
        }
        pixels
    }
}
