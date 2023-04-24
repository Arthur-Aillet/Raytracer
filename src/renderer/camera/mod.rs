//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// camera
//

use crate::vectors;

use super::renderer_common::Transform;
use vectors::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Lens {
    pub height: i64,
    pub width: i64,
    pub distance: f64,
    pub vector_to_first_pixel: Vector,
}

impl Lens  {
    pub fn default() -> Lens {
        Lens {
            height: 1080,
            width: 1920,
            distance : 0.0,
            vector_to_first_pixel: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub transform: Transform,
    pub lens: Lens,
    pub fov : i64,
    pub smooth_shadow: bool,
    pub smooth_shadow_step: i16,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
}

impl Camera {
    pub fn default() -> Camera {
        let mut camera = Camera {
            transform: Transform::default(),
            lens: Lens::default(),
            fov: 60,
            smooth_shadow: false,
            smooth_shadow_step: 0,
            diffuse: 0.7,
            ambient: 0.3,
            specular: 0.3,
        };
        camera.calculate_lens_distance();
        let vector_director = Vector {x: 0.0, y: camera.lens.distance, z: 0.0};
        camera.lens.vector_to_first_pixel = Vector {x: camera.transform.pos.x, y: camera.transform.pos.y, z: camera.transform.pos.z};
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + Vector {x:0.0, y:0.0, z:1.0} * (camera.lens.height as f64 / 2.0);
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + vector_director;
        camera.lens.vector_to_first_pixel = camera.lens.vector_to_first_pixel + Vector {x: -1.0, y: 0.0, z: 0.0} * (camera.lens.width as f64 / 2.0);
        camera
    }

    pub fn get_pixel_vector(&self, x: i64, y: i64) -> Vector {
        let mut pixel_vector = self.lens.vector_to_first_pixel.clone();

        pixel_vector = pixel_vector + Vector {x:1.0, y:0.0, z:0.0} * x as f64;
        pixel_vector = pixel_vector + Vector {x:0.0, y:0.0, z:-1.0} * y as f64;
        pixel_vector.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        pixel_vector.normalize()
    }

    pub(crate) fn calculate_lens_distance(&mut self) {
        self.lens.distance = (self.lens.width as f64 / 2.0) / (self.fov as f64 / 2.0).to_radians().tan();
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
