//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// Main
//

use renderer::Renderer;

mod ppm_interface;
mod nannou;
mod vectors;
mod matrix;
mod renderer;
mod config;

use std::env;
use std::io;

fn graphical_loop(config: &config::Config) {
    let mut app = nannou::NannouInterface::new(config.width, config.height);
    app.run();
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::from_args(&args);
    let mut renderer: Renderer = Renderer::get_renderer_from_file(&config.config_file, config.height, config.width);

    if config.g_flag {
        graphical_loop(&config);
    } else {
        let mut ppm = ppm_interface::PPMInterface::new(&config.save_file);
        ppm.write(config.width, config.height, renderer.render());
    }

    Ok(())
}

