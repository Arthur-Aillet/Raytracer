//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer
//

use vectors::VectorF;
use crate::vectors;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    pos: Point,
    rotation : Rotation,
    scale : Scale,
}

struct Lens {
    distance : f64,
    height : i64,
    width : i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    transform : Transform,
    lens : Lens,
    fov : i16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sphere {
    transform : Transform,
    color : Color
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Primitives {
    spheres: Vec<Sphere>,
    lights: Vec<Lights>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Renderer {
    camera: Camera,
    primitives : Primitives,
}

impl Camera {
    fn get_pixel_vector(&self, x:i64, y:i64) -> VectorF {
        vectors::VectorF {
            origin : vectors::Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            direction: vectors::Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    fn calulate_lens_distance(&mut self) {
        self.lens.distance = (self.lens.height as f64 / 2.0) / (self.fov as f64).tan();
    }
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
        }
    }

    pub fn render() -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        pixels
    }
}
