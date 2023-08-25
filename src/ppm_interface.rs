//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module
//

use std::fs::File;
use std::io::{BufWriter, Write};

pub struct PPMInterface {
    file: File,
}

impl PPMInterface {
    pub fn new(file_path: &String) -> Self {
        std::fs::create_dir_all(".raytracer/").expect("Invalid File Path");
        let file = File::create(file_path).expect("Invalid File Path");
        PPMInterface { file }
    }

    fn create_header(&self, width: i64, height: i64) -> String {
        format!("P6\n{} {}\n255\n", width, height)
    }

    pub fn write(&mut self, width: i64, height: i64, content: Vec<u8>) {
        let header = self.create_header(width, height);
        let mut writer = BufWriter::new(&self.file);

        writer.write_all(header.as_bytes()).unwrap();
        writer.write_all(&content).unwrap();
    }
}
