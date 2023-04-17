//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

mod ppm_interface;

use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut ppm = ppm_interface::PPMInterface::new(String::from(args[1].clone()));

    ppm.write(1000, 1000);
    Ok(())
}


