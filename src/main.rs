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
use serde;
use serde_json;
use std::fs::File;

use std::env;

fn main() -> std::io::Result<()> {
    let json_file_path = Path::new("../ex.json");
    let file = File::open(json_file_path);

    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let mut ppm = ppm_interface::PPMInterface::new(String::from(args[1].clone()));
    let mut content : Vec<u8> = Vec::new();
    let height = 1000;
    let width = 1000;

    for h in 0..height {
        for w in 0..width {
            if w % 50 >= 25 {
                if h % 50 >= 25 {
                    content.extend(&[0xFF, 0x00, 0x00]);
                } else {
                    content.extend(&[0x00, 0xFF, 0xFF]);
                }
            } else {
                if h % 50 >= 25 {
                    content.extend(&[0xFF, 0xFF, 0x00]);
                } else {
                    content.extend(&[0x00, 0xFF, 0x00]);
                }
            }
        }
    }

    ppm.write(width, height, content);
    Ok(())
}


