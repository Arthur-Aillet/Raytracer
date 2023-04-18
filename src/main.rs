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
use std::io::Read;
use librustconfig::config::{Config, OptionType};

use std::env;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    Ok(())
}


