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

use std::fs;
use librustconfig::config::Config;

// fn main() -> std::io::Result<()> {
//     let mut file = std::fs::File::open("config_file/ex.cfg").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     print!("{}", contents);

//     Ok(())
// }

fn main() {
    let mut cfg = Config::new();
    if cfg.load_from_string(
        "section1 : {
            integer_value = -12;
            boolean_value = true;
            int64_value = 99999L;
            float_value = 0.9999991;
            string_value = \"test string value \";
        }"
    ).is_err() {
        panic!("Can\t load configuration from string value!");
    }
}



