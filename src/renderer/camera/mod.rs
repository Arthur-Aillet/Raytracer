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

    pub fn get_pixel_vector(&self, x: i64, y: i64) -> Vector {
        let mut pixel_vector = self.lens.vector_to_first_pixel.clone();

        pixel_vector = pixel_vector + Vector {x:1.0, y:0.0, z:0.0} * x as f64;
        pixel_vector = pixel_vector + Vector {x:0.0, y:0.0, z:-1.0} * y as f64;
        pixel_vector.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        pixel_vector.normalize()
    }
// Point { x: -960.0, y: 441.91302184715596, z: 540.0 } }
    pub fn calculate_lens_distance(&mut self) {
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
