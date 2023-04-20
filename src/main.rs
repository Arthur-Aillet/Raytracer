//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

use json_parser::Renderer;
use std::fs;
use serde_json::{Result, Value};


mod ppm_interface;
mod vectors;
mod matrix;
mod json_parser;


fn main() {
    let mut renderer : Renderer = Renderer::new();
    renderer.get_renderer_from_file("src/json_parser/ex.json".to_string());
    let data = fs::read_to_string("src/json_parser/ex.json").expect("Unable to read file");
    let _json: Value = serde_json::from_str(&data.to_string()).unwrap();
    print!("test:{}\n", renderer.camera.height);
}
