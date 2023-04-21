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

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Texture {
    pub texture_type: u64,
    pub color: Color,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Texture {
    fn texture(&self, _x: f64, _y: f64) -> Color {
        if self.texture_type == 1 {
            self.color
        } else {self.color}
    }
}
