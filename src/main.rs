//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

mod ppm_interface;
mod renderer;
mod vectors;
mod matrix;

use std::path::Path;
use std::fs::File;

use std::env;
use crate::renderer::Renderer;

fn main() -> std::io::Result<()> {
    let json_file_path = Path::new("../ex.json");
    let file = File::open(json_file_path);
    
    let args: Vec<String> = env::args().collect();
    let height = 1080;
    let width = 1920;

    for fov in (60..90).step_by(3) {
        let mut ppm = ppm_interface::PPMInterface::new([String::from(args[1].clone()), fov.to_string()].join("-"));
        let renderer = Renderer::new(fov);
        ppm.write(width, height, renderer.render());
    }
    Ok(())
}


