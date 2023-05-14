//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

use raytracer::{config, sfml_interface};
use renderer::Renderer;

mod ppm_interface;
mod vectors;
mod matrix;
mod renderer;
mod nannou_interface;

use std::env;

fn print_help() {
    let config = config::Config::new();
    println!("USAGE: ./rustracer [OPTIONS]\n");
    println!("OPTIONS:");
    println!("\t--help\t\t\tDisplay this help");
    println!("\t-j <JSON_FILE>\t\tSpecify the config file");
    println!("\t-s <PPM_FILE>\t\tSpecify the save file");
    println!("\t-w <WIDTH>\t\tSpecify the width of the image");
    println!("\t-h <HEIGHT>\t\tSpecify the height of the image");
    println!("\t-g <GRAPHIC MODE>\tDisplay the image in a window");
    println!("\t--layout \t\tDisplay the layout");
    println!("\t-f <FAST MODE>\t\tDisplay the image with reduce quality");

    println!("\n\n\t\t+----------------+\n");

    println!("<JSON_FILE>: The file must be a valid JSON file");
    println!("<PPM_FILE>: The file must be a valid PPM file");
    println!("<WIDTH>: The width must be a positive integer");
    println!("<HEIGHT>: The height must be a positive integer");
    println!("<GRAPHIC MODE>: The graphic mode must be a positive integer:");
    println!("\t- 0: PPM");
    println!("\t- 1: Nannou");
    println!("\t- 2: SFML");
    println!("<FAST MODE>: The fast mode must be a positive integer");

    println!("\n\n\t\t+----------------+");

    println!("\t\t| Default values |");
    config.print();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::from_args(&args);
    let renderer: Option<Renderer> = Renderer::get_renderer_from_file(&config);

    if config.help {
        print_help();
        return Ok(());
    }

    match config.graphic {
        0 => ppm_interface::PPMInterface::new(&config.save_file).write(config.width, config.height, renderer.unwrap().render(&config)),
        1 => nannou_interface::NannouInterface::new(config.width, config.height).run(),
        2 => sfml_interface::SfmlInterface::new(config).run(),
        _ => println!("Error: Invalid graphic mode"),
    }


    Ok(())
}
