//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// mod
//

mod vectors;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    transform : Transform,
    height: i64,
    width: i64,
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

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
        }
    }

    pub fn render() -> Vec<u8> {
        let mut pixels = Vec::new();
    }
}
