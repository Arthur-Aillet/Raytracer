//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module
//

use std::fs::File;
use std::io::{BufWriter, Write};

pub struct NannouInterface {
    height: i64,
    width: i64,
    vec_pixels: Vec<u8>,
}

impl NannouInterface {
    pub fn new(width: i64, height: i64) -> Self {
        let vec_pixels = vec![0; (width * height * 3) as usize];
        NannouInterface {
            height,
            width,
            vec_pixels,
        }
    }

    pub fn write(&mut self, x: i64, y: i64, color: Vec<u8>) {
        let index = ((y * self.width + x) * 3) as usize;
        self.vec_pixels[index] = color[0];
    }

    pub fn get_pixels(&self) -> &Vec<u8> {
        &self.vec_pixels
    }
}