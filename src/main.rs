//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// main
//

// use json_parser::Renderer;
use std::fs;
use serde_json::{Result, Value};


mod ppm_interface;
mod renderer;
mod vectors;
mod matrix;
// mod json_parser;

// fn main() -> std::io::Result<()> {
//     let mut file = std::fs::File::open("config_file/ex.cfg").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     print!("{}", contents);

//     Ok(())
// }

// fn main() {
    // let renderer : Renderer;
    // renderer.get_renderer_from_file("src/jsom_parser/ex.json".to_string());
    // print!("test:{}", renderer.camera.height)
    // let data = fs::read_to_string("src/config_file/ex.json").expect("Unable to read file");
    // let json: Value = serde_json::from_str(&data.to_string()).unwrap();
    // for element in json["primitives"]["spheres"].as_array() {
    //     print!("{}\n", element[0]);
    // }
    fn main() {
        let data = fs::read_to_string("src/config_file/ex.json").expect("Unable to read file");
        let v: Value = serde_json::from_str(&data.to_string()).unwrap();
        println!("{}", v);
    }
// }
