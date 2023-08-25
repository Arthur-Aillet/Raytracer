//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

use raytracer::config;
use renderer::Renderer;

mod matrix;
mod nannou_interface;
mod ppm_interface;
mod renderer;
mod vector;

use std::env;

fn print_help() {
    let config = config::Config::default();
    println!("USAGE: ./rustracer [OPTIONS]\n");
    println!("OPTIONS:");
    println!("\t--help\t\t\tDisplay this help");
    println!("\t-j <JSON_FILE>\t\tSpecify the config file");
    println!("\t-s <PPM_FILE>\t\tSpecify the save file");
    println!("\t-w <WIDTH>\t\tSpecify the width of the image");
    println!("\t-h <HEIGHT>\t\tSpecify the height of the image");
    println!("\t--graphic\t\tDisplay the image in a live graphic mode");
    println!("\t--layout \t\tDisplay the layout");
    println!("\t-f <FAST MODE>\t\tDisplay the image with reduce quality");

    println!("\n\n\t\t+----------------+\n");

    println!("<JSON_FILE>: The file must be a valid JSON file");
    println!("<PPM_FILE>: The file must be a valid PPM file");
    println!("<WIDTH>: The width must be a positive integer");
    println!("<HEIGHT>: The height must be a positive integer");
    println!("<FAST MODE>: The fast mode must be a positive integer");

    println!("\n\n\t\t+----------------+");

    println!("\t\t| Default values |");
    config.print();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::from_args(&args);

    if config.help {
        print_help();
        return Ok(());
    }

    let renderer: Option<Renderer> = Renderer::get_renderer_from_file(&config);
    if renderer.is_none() {
        std::process::exit(84);
    }

    if config.graphic {
        nannou_interface::run_nannou_interface();
    } else {
        ppm_interface::PPMInterface::new(&config.save_file).write(
            config.width,
            config.height,
            renderer.unwrap().render(&config),
        )
    }

    Ok(())
}
