//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

use json_parser::Renderer;

mod ppm_interface;
mod vectors;
mod matrix;
mod json_parser;

use std::path::Path;
use std::fs::File;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut ppm = ppm_interface::PPMInterface::new(String::from(args[1].clone()));
    let height = 1080;
    let width = 1920;
    let mut renderer : Renderer = Renderer::new();
    renderer.get_renderer_from_file("src/json_parser/ex.json".to_string());

    for object in renderer.primitives.iter_mut() {
        print!("{}\n", object.obj_type());
    }

    ppm.write(width, height, renderer.render());
    Ok(())
}
