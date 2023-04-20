//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

use renderer::Renderer;

mod ppm_interface;
mod vectors;
mod matrix;
mod renderer;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut ppm = ppm_interface::PPMInterface::new(String::from(args[1].clone()));
    let height = 1080;
    let width = 1920;
    let mut renderer : Renderer = Renderer::new();
    renderer.get_renderer_from_file(String::from(args[2].clone()));
    ppm.write(width, height, renderer.render());
    Ok(())
}
