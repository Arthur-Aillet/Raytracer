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
use serde_json;
use serde;

use std::env;

fn main() -> std::io::Result<()> {
    let json_file_path = Path::new("../ex.json");
    let file = File::open(json_file_path);

    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let mut ppm = ppm_interface::PPMInterface::new(String::from(args[1].clone()));

    ppm.write(1000, 1000);
    Ok(())
}


