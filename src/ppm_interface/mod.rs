//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module
//

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub struct PPMInterface {
    file: File,
    content: Vec<u8>,
}

impl PPMInterface {
    pub fn new(file_path: String) -> PPMInterface {
        let file = File::create(file_path).unwrap();
        let content = Vec::new();

        PPMInterface {
            file,
            content
        }
    }

    fn create_header(&mut self, width: u32, height: u32) {
        self.content.extend(format!("P6\n{} {}\n255\n", width, height).as_bytes());
    }

    fn write_white_pixel(&mut self) {
        self.content.extend(&[0xFF, 0xFF, 0xFF]);
    }

    fn write_black_pixel(&mut self) {
        self.content.extend(&[0x00, 0x00, 0x00]);
    }

    pub fn write(&mut self, width: u32, height: u32) {
        self.create_header(width, height);
        for _ in 0..height {
            for count in 0..width {
                if count % 2 == 0 {
                    self.write_white_pixel();
                } else {
                    self.write_black_pixel();
                }
            }
        }

        let mut writer = BufWriter::new(&self.file);
        writer.write_all(&self.content).unwrap();
    }
}
