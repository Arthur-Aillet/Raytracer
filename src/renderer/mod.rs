//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer
//

pub mod primitives;

use std::f64::INFINITY;
use crate::vectors;
use vectors::Vector;
use crate::renderer::primitives::{Object, Sphere, Light, Intersection};

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
    objects: Vec<Sphere>,
    lights: Vec<Light>,
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
            transform: Transform::new(0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0, 0.0),
            fov: 80,
            diffuse: 0.7,
            ambient: 0.3,
            specular: 0.6,
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
        result
    }

    fn get_pixel_vector(&self, x: i64, y: i64) -> Vector {
        let mut pixel_vector = self.lens.vector_to_first_pixel.clone();

        pixel_vector = pixel_vector + Vector {x:1.0, y:0.0, z:0.0} * x as f64;
        pixel_vector = pixel_vector + Vector {x:0.0, y:0.0, z:-1.0} * y as f64;
        pixel_vector.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        pixel_vector.normalize()
    }

    fn calculate_lens_distance(&mut self) {
        self.lens.distance = (self.lens.height as f64 / 2.0) / (self.fov as f64 / 2.0).to_radians().tan();
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
            objects: vec![
                Sphere {
                    origin: Vector {x:2.0, y:3.0, z:1.0},
                    radius: 1.0,
                        ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    }
                }, Sphere {
                    origin: Vector {x:-2.0, y:3.0, z:-1.0},
                    radius: 1.0,
                    ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    }
                },
                Sphere {
                    origin: Vector {x:-2.0, y:6.0, z:0.5},
                    radius: 1.0,
                    ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    }
                },                Sphere {
                    origin: Vector {x:2.0, y:6.0, z:-0.5},
                    radius: 1.0,
                    ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    }
                },                Sphere {
                    origin: Vector {x:-2.0, y:9.0, z:0.7},
                    radius: 1.0,
                    ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    }
                },                Sphere {
                    origin: Vector {x:2.0, y:9.0, z:-0.7},
                    radius: 1.0,
                    ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    }
                },
            ],
            lights: vec![ Light {
                origin: Vector {x:0.0, y:3.0, z:0.0},
                intensity: 1000.0,
                color: Vector {
                    x: 1.0,
                    y: 0.7,
                    z: 1.0,
                }
            }, Light {
                origin: Vector {x:0.0, y:0.0, z:0.0},
                intensity: 1000.0,
                color: Vector {
                    x: 0.1,
                    y: 0.1,
                    z: 1.0,
                }
            }, Light {
                origin: Vector {x:0.0, y:7.0, z:0.0},
                intensity: 1000.0,
                color: Vector {
                    x: 0.1,
                    y: 1.0,
                    z: 5.0,
                }
            },
            ]
        }
    }

    fn calculate_light(&self, light: &Light, intersect: Intersection, camera_to_pixel: Vector, object: Sphere) -> Vector {
        let light_vector = (light.origin - intersect.intersection_point).normalize();
        let normal_vector = intersect.normal.normalize();

        let diffuse = light_vector.dot_product(normal_vector).max(0.0) * self.camera.diffuse * object.diffuse;

        let reflected = light_vector.reflect(normal_vector).normalize();
        let view = (camera_to_pixel * -1.0).normalize();
        let specular = self.camera.specular * object.specular * reflected.dot_product(view).max(0.0).powf(object.shininess);

        object.color * light.color * diffuse + light.color * specular
    }

    fn found_nearest_intersection(&self, camera_to_pixel: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

        for object in self.objects.iter() {
            let intersect = object.intersection(camera_to_pixel, self.camera.transform.pos);

            if intersect != None {

                let distance_found = (intersect.unwrap().intersection_point - self.camera.transform.pos).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some (Intersection{
                        intersection_point: intersect.unwrap().intersection_point,
                        normal: intersect.unwrap().normal,
                        object: Some(*object)
                    });
                }
            }
        }
        found_intersection
    }

    pub fn render(&self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {
                let camera_to_pixel = self.camera.get_pixel_vector(j, i);
                let intersect = self.found_nearest_intersection(camera_to_pixel);
                if intersect != None {
                    let mut color = intersect.unwrap().object.unwrap().color * self.camera.ambient * intersect.unwrap().object.unwrap().ambient;
                    for light in self.lights.iter() {
                        color = color + self.calculate_light(light, intersect.unwrap(), camera_to_pixel, intersect.unwrap().object.unwrap());
                    }
                    pixels.extend(&[
                        ((color.x).clamp(0.0, 1.0) * 255.0) as u8,
                        ((color.y).clamp(0.0, 1.0) * 255.0) as u8,
                        ((color.z).clamp(0.0, 1.0) * 255.0) as u8
                    ]);
                } else {
                    pixels.extend(&[
                        (0 as f64) as u8,
                        (0 as f64) as u8,
                        (0 as f64) as u8
                    ]);
                }
            }
        }
        pixels
    }
}
