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

fn print_help() {
    let config = config::Config::new();
    println!("USAGE: ./rustracer [OPTIONS]\n");
    println!("OPTIONS:");
    println!("\t--help\t\t\tDisplay this help");
    println!("\t-j <JSON_FILE>\t\tSpecify the config file");
    println!("\t-s <PPM_FILE>\t\tSpecify the save file");
    println!("\t-w <WIDTH>\t\tSpecify the width of the image");
    println!("\t-h <HEIGHT>\t\tSpecify the height of the image");
    println!("\t-g\t\t\tDisplay the image in a window");
    println!("\n\n\t\t+----------------+");
    println!("\t\t| Default values |");
    config.print();
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::from_args(&args);
    let mut renderer: Renderer = Renderer::get_renderer_from_file(&config);

    if config.help {
        print_help();
        return Ok(());
    }
    config.print();

    if config.graphic {
        let mut app = nannou::NannouInterface::new(config.width, config.height);
        app.run();
    } else {
        let mut ppm = ppm_interface::PPMInterface::new(&config.save_file);
        ppm.write(config.width, config.height, renderer.render(&config));
    }

    Ok(())
}

