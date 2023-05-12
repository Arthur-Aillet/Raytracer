//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// common structures to the renderer
//

use crate::vectors;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

use std::ops::{Add, Mul, Sub};
use vectors::Vector;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy)]
pub enum Textures_types {
    COLOR,
    GRADIENT,
    PERLIN,
    CHECKERS,
    IMAGE,
}

impl Textures_types {
    pub fn from_u64(src: u64) -> Textures_types {
        match src {
            0 => Textures_types::COLOR,
            1 => Textures_types::GRADIENT,
            2 => Textures_types::PERLIN,
            3 => Textures_types::CHECKERS,
            4 => Textures_types::IMAGE,
            _ => Textures_types::COLOR,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Deserialize, Serialize)]
pub struct Transform {
    pub pos: Vector,
    pub rotation: Vector,
    pub scale: f64,
}

impl Transform {
    pub fn default() -> Transform {
        Transform {
            pos: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: 1.0,
        }
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.rotation == other.rotation && self.scale == other.scale
    }
}

impl Add<Transform> for Transform {
    type Output = Transform;
    fn add(self, other: Transform) -> Transform {
        Transform {
            pos: self.pos + other.pos,
            rotation: self.rotation + other.rotation,
            scale: self.scale + other.scale,
        }
    }
}

impl Sub<Transform> for Transform {
    type Output = Transform;
    fn sub(self, other: Transform) -> Transform {
        Transform {
            pos: self.pos - other.pos,
            rotation: self.rotation - other.rotation,
            scale: self.scale - other.scale,
        }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, other: Transform) -> Transform {
        Transform {
            pos: self.pos * other.pos,
            rotation: self.rotation * other.rotation,
            scale: self.scale * other.scale,
        }
    }
}

#[derive(Deserialize, Serialize)]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn default() -> Color {
        Color {
            r: 255.0,
            g: 255.0,
            b: 255.0,
        }
    }

    pub fn as_vector(self) -> Vector {
        Vector {
            x: self.r / 255.0,
            y: self.g / 255.0,
            z: self.b / 255.0,
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Image {
    pub vector: Vec<Color>,
    pub width: i64,
    pub height: i64,
}

impl Image {
    pub fn default() -> Image {
        Image {
            vector: Vec::new(),
            height: 0,
            width: 0,
        }
    }
}

#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
pub struct Texture {
    pub texture_type: u64,
    pub color: Color,
    pub secondary_color: Color,
    pub image: Image,
    pub mod1: f64,
    pub mod2: f64,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub metalness: f64,
    pub shininess: f64,
    pub roughness: f64,
    pub sampling_ponderation: f64,
}

impl Texture {
    pub fn default() -> Texture {
        Texture {
            texture_type: Textures_types::COLOR as u64,
            color: Color::default(),
            secondary_color: Color::default(),
            image: Image::default(),
            mod1: 2.0,
            mod2: 2.0,
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.4,
            metalness: 0.1,
            shininess: 4.0,
            roughness: 0.25,
            sampling_ponderation: 1.0,
        }
    }

    fn gradient_color(&self, _u: f64, v: f64) -> Color {
        Color {
            r: self.color.r + if self.color.r - self.secondary_color.r < 0.0 {(self.color.r - self.secondary_color.r) * v} else {((self.color.r - self.secondary_color.r) * v) * -1.0},
            g: self.color.g + if self.color.g - self.secondary_color.g < 0.0 {(self.color.g - self.secondary_color.g) * v} else {((self.color.g - self.secondary_color.g) * v) * -1.0},
            b: self.color.b + if self.color.b - self.secondary_color.b < 0.0 {(self.color.b - self.secondary_color.r) * v} else {((self.color.b - self.secondary_color.b) * v) * -1.0},
        }
    }

    fn perlin_noise(&self, u: f64, v: f64) -> Color {
        let mut rand = StdRng::seed_from_u64(self.mod1 as u64);
        let mut vectors: Vec<Vector> = Vec::new();

        for _ in 0..(self.mod2 as u64).pow(2) {
            vectors.push(Vector {
                x: rand.gen_range(-1.0..1.0),
                y: rand.gen_range(-1.0..1.0),
                z: 0.0,
            })
        }
        let gradiants: [Vector; 4] = [
            vectors[((u * self.mod2) % self.mod2) as usize % vectors.len()],
            vectors[((u * self.mod2 + 1.0) % self.mod2) as usize % vectors.len()],
            vectors[((((v * self.mod2) % self.mod2) * self.mod2) + ((u * self.mod2) % self.mod2)) as usize % vectors.len()],
            vectors[((((v * self.mod2) % self.mod2) * self.mod2) + ((u * self.mod2 + 1.0) % self.mod2)) as usize % vectors.len()],
            ];
        let distances: [Vector; 4] = [
            Vector {x: 0.0, y: 0.0, z: 0.0} - Vector {x: (u % (1.0 / self.mod2)) * self.mod2, y: (v % (1.0 / self.mod2)) * self.mod2, z: 0.0},
            Vector {x: 0.0, y: 1.0, z: 0.0} - Vector {x: (u % (1.0 / self.mod2)) * self.mod2, y: (v % (1.0 / self.mod2)) * self.mod2, z: 0.0},
            Vector {x: 1.0, y: 0.0, z: 0.0} - Vector {x: (u % (1.0 / self.mod2)) * self.mod2, y: (v % (1.0 / self.mod2)) * self.mod2, z: 0.0},
            Vector {x: 1.0, y: 1.0, z: 0.0} - Vector {x: (u % (1.0 / self.mod2)) * self.mod2, y: (v % (1.0 / self.mod2)) * self.mod2, z: 0.0},
        ];
        let i1 = gradiants[0].dot_product(distances[0]);
        let i2 = gradiants[1].dot_product(distances[1]);
        let i3 = gradiants[2].dot_product(distances[2]);
        let i4 = gradiants[3].dot_product(distances[3]);
        let result = (i1 + u * (i2 - i1)) + v * ((i3 + u * (i4 - i3)) - (i1 + u * (i2 - i1)));
        Color {
            r: self.color.r + result * (self.secondary_color.r - self.color.r),
            g: self.color.g + result * (self.secondary_color.g - self.color.g),
            b: self.color.b + result * (self.secondary_color.b - self.color.b),
        }
    }

    fn checkers_color(&self, width: f64, height: f64, u: f64, v: f64) -> Color {
        if ((u * width) as i64 + (v * height) as i64) % 2 == 0 {
            self.color
        } else {
            self.secondary_color
        }
    }

    fn image_color(&self, width: f64, height: f64, u: f64, v: f64) -> Color {
        Color::default()
    }

    pub fn texture(&self, x: f64, y: f64) -> Color {
        match Textures_types::from_u64(self.texture_type) {
            Textures_types::COLOR => self.color,
            Textures_types::GRADIENT => self.gradient_color(x, y),
            Textures_types::PERLIN => self.perlin_noise(x, y),
            Textures_types::CHECKERS => return self.checkers_color(self.mod1, self.mod2, x, y),
            Textures_types::IMAGE => self.image_color(self.mod1, self.mod2, x, y),
        }
    }
}
