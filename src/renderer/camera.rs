//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// camera
//

use crate::vector;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::types::Transform;
use vector::Vector;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Lens {
    pub height: i64,
    pub width: i64,
    pub distance: f64,
    pub vector_to_first_pixel: Vector,
}

impl Lens {
    pub fn default(height: i64, width: i64) -> Lens {
        Lens {
            height,
            width,
            distance: 0.0,
            vector_to_first_pixel: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Camera {
    pub transform: Transform,
    pub lens: Lens,
    pub fov: i64,
    pub smooth_shadow: bool,
    pub smooth_shadow_step: i16,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub shadow_bias: f64,
    pub aces_tone_mapping: bool,
    pub recursivity: i64,
    pub reflection_samples: i64,
    pub threads: u64,
    pub progression: bool,
    pub image_buffer_size: u64,
    pub super_sampling: u64,
    pub super_sampling_precision: u64,
    pub reflecion_samples: f64,
    pub display_normals: bool,
    pub display_location: bool,
    pub display_dot_product: bool,
}

impl Camera {
    pub fn default(height: i64, width: i64) -> Camera {
        let mut camera = Camera {
            transform: Transform::default(),
            lens: Lens::default(height, width),
            fov: 60,
            smooth_shadow: false,
            smooth_shadow_step: 0,
            diffuse: 0.7,
            ambient: 0.3,
            specular: 0.3,
            shadow_bias: 1e-14,
            aces_tone_mapping: true,
            recursivity: 5,
            reflection_samples: 5,
            threads: 8,
            progression: false,
            image_buffer_size: 1,
            super_sampling: 5,
            super_sampling_precision: 10,
            reflecion_samples: 16.0,
            display_normals: false,
            display_location: false,
            display_dot_product: false,
        };
        camera.calculate_lens_distance();
        camera.calculate_lens_size();
        camera
    }

    pub fn get_random_pixel_vector(&self, x: i64, y: i64) -> Vector {
        let mut rng = rand::thread_rng();
        let mut pixel_vector = self.lens.vector_to_first_pixel;

        pixel_vector = pixel_vector
            + Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            } * (x as f64 + rng.gen_range(0.0..1.0) - 0.5);
        pixel_vector = pixel_vector
            + Vector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            } * (y as f64 + rng.gen_range(0.0..1.0) - 0.5);
        pixel_vector.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
        pixel_vector.normalize()
    }

    fn get_pixel_vector(&self, x: f64, y: f64) -> Vector {
        let mut _rng = rand::thread_rng();
        let mut pixel_vector = self.lens.vector_to_first_pixel;

        pixel_vector = pixel_vector
            + Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            } * x;
        pixel_vector = pixel_vector
            + Vector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            } * y;
        pixel_vector.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
        pixel_vector.normalize()
    }

    pub fn get_pixel_vectors(&self, x: i64, y: i64, n: u64) -> Vec<Vector> {
        let mut result: Vec<Vector> = Vec::new();

        if n <= 1 {
            result.push(self.get_pixel_vector(x as f64, y as f64))
        }
        if (2..=4).contains(&n) {
            for _i in 0..n {
                result.push(self.get_random_pixel_vector(x, y));
            }
        }
        if n > 4 {
            result.push(self.get_pixel_vector(x as f64 - 0.5, y as f64 - 0.5));
            result.push(self.get_pixel_vector(x as f64 - 0.5, y as f64 + 0.5));
            result.push(self.get_pixel_vector(x as f64 + 0.5, y as f64 - 0.5));
            result.push(self.get_pixel_vector(x as f64 + 0.5, y as f64 + 0.5));
        }
        result
    }

    pub(crate) fn calculate_lens_size(&mut self) {
        let vector_director = Vector {
            x: 0.0,
            y: self.lens.distance,
            z: 0.0,
        };

        self.lens.vector_to_first_pixel = Vector {
            x: self.transform.pos.x,
            y: self.transform.pos.y,
            z: self.transform.pos.z,
        };
        self.lens.vector_to_first_pixel = self.lens.vector_to_first_pixel
            + Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            } * (self.lens.height as f64 / 2.0);
        self.lens.vector_to_first_pixel = self.lens.vector_to_first_pixel + vector_director;
        self.lens.vector_to_first_pixel = self.lens.vector_to_first_pixel
            + Vector {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            } * (self.lens.width as f64 / 2.0);
    }

    pub(crate) fn calculate_lens_distance(&mut self) {
        self.lens.distance =
            (self.lens.width as f64 / 2.0) / (self.fov as f64 / 2.0).to_radians().tan();
    }

    pub fn aces_curve(self, val: f64) -> f64 {
        if self.aces_tone_mapping {
            let a = 2.51;
            let b = 0.03;
            let c = 2.43;
            let d = 0.59;
            let e = 0.14;

            ((val * (a * val + b)) / (val * (c * val + d) + e)).clamp(0.0, 1.0)
        } else {
            val.clamp(0.0, 1.0)
        }
    }
}
