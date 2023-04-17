//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// mod
//

mod vectors;

pub struct Camera {
    pos: Point,
    height: i64,
    width: i64,
}

pub struct Renderer {
    camera: Camera,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
        }
    }

    pub fn render() -> Vec<u8> {
        let mut pixels = Vec::new();
    }
}
