//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module
//

use nannou::prelude::*;
use nannou::Frame;
use nannou::sketch;
use nannou::App;

use crate::renderer::Renderer;
use crate::config;
use std::env;

pub struct NannouInterface {
    height: i64,
    width: i64,
    vec_pixels: Vec<u8>,
}

fn view(app: &App, frame: Frame) {
    let args: Vec<String> = env::args().collect();
    let draw = app.draw();
    let config = config::Config::from_args(&args);
    let mut renderer: Renderer = Renderer::get_renderer_from_file(&config.config_file, config.height, config.width);

    draw.background().color(BLUE);
    let pixels = renderer.render();
    let mut index = 0;
    println!("width: {}, height: {}", config.width, config.height);
    for y in (-config.height/2)..(config.height/2) {
        for x in (-config.width/2)..(config.width/2) {
            let color = pixels[index..index + 3].to_vec();
            draw.rect()
                .x_y(x as f32, -y as f32)
                .w_h(1.0, 1.0)
                .color(Rgb::new(color[0] as f32 / 255.0, color[1] as f32 / 255.0, color[2] as f32 / 255.0));
            index += 3;
        }
    }
    println!("index: {}", index);
    draw.to_frame(app, &frame).unwrap();
    println!("done");
}

impl NannouInterface {

    pub fn new(width: i64, height: i64) -> Self {
        let vec_pixels = vec![0; (width * height * 3) as usize];
        NannouInterface {
            height,
            width,
            vec_pixels,
        }
    }

    pub fn run(&self) {
        nannou::sketch(view).run();
    }

    pub fn write(&mut self, x: i64, y: i64, color: Vec<u8>) {
        let index = ((y * self.width + x) * 3) as usize;
        self.vec_pixels[index] = color[0];
    }

    pub fn get_pixels(&self) -> &Vec<u8> {
        &self.vec_pixels
    }
}