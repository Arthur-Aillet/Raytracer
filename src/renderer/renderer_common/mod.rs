//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// common structures to the renderer
//

use crate::vectors;

use vectors::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vector,
    pub rotation: Vector,
    pub scale: Vector,
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
            scale: Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    pub texture_type: u64,
    pub color: Color,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub metalness: f64,
    pub shininess: f64,
    pub roughness: f64,
    pub diffuse_roughness: f64,
}

impl Texture {
    pub fn default() -> Texture {
        Texture {
            texture_type: 1,
            color: Color::default(),
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.4,
            metalness: 0.1,
            shininess: 4.0,
            roughness: 0.25,
            diffuse_roughness: 1.0,
        }
    }

    pub fn texture(&self, _x: f64, _y: f64) -> Color {
        if self.texture_type == 1 {
            self.color
        } else {self.color}
    }
}
