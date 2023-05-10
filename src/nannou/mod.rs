// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module

use nannou::prelude::*;
use nannou::Frame;
use nannou::App;

use crate::renderer::Renderer;
use crate::config;
use std::env;

pub struct Model {
    window: WindowId,
    config: Config,
}

fn model(app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    let mut config = config::Config::from_args(&args);
    let window = app
        .new_window()
        .title("Rustracer")
        .size(config.width as u32, config.height as u32)
        .view(view)
        .build()
        .expect("Failed to build the window");

    config.height = config.height / if config.fast_mode == 0 { 1 } else { config.fast_mode };
    config.width = config.width / if config.fast_mode == 0 { 1 } else { config.fast_mode };
    Model {
        window,
        config,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn handle_keys(app: &App) {
    if app.keys.down.contains(&Key::Left) {
        println!("Left");
    }
    if app.keys.down.contains(&Key::Right) {
        println!("Right");
    }
    if app.keys.down.contains(&Key::Up) {
        println!("Up");
    }
    if app.keys.down.contains(&Key::Down) {
        println!("Down");
    }
}

fn view(_app: &App, model: &Model, frame: Frame) {
    handle_keys(&_app);
    let renderer = Renderer::get_renderer_from_file(&model.config);
    let pixels = renderer.render(&model.config);
    let mut index = 0;

    let window = _app.window_rect();
    let view = window.pad(100.0);

    let draw = _app.draw();
    draw.background().color(BLUE);
    for y in (-model.config.height / 2)..(model.config.height / 2) {
        for x in (-model.config.width / 2)..(model.config.width / 2) {
            let color = pixels[index..index + 3].to_vec();
            draw.rect()
                .x_y((x as f32) * (if model.config.fast_mode == 0 { 1 } else { model.config.fast_mode })as f32, (-y as f32) * (if model.config.fast_mode == 0 { 1 } else { model.config.fast_mode }) as f32)
                .w_h((if model.config.fast_mode == 0 { 1 } else { model.config.fast_mode }) as f32, (if model.config.fast_mode == 0 { 1 } else { model.config.fast_mode }) as f32)
                .color(Rgb::new(
                    color[0] as f32 / 255.0,
                    color[1] as f32 / 255.0,
                    color[2] as f32 / 255.0,
                ));
            index += 3;
        }
    }
    draw.to_frame(_app, &frame).unwrap();
}

pub struct NannouInterface {
    height: i64,
    width: i64,
    vec_pixels: Vec<u8>,
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
        nannou::app(model).update(update).run();
    }

    pub fn write(&mut self, x: i64, y: i64, color: Vec<u8>) {
        let index = ((y * self.width + x) * 3) as usize;
        self.vec_pixels[index..index + 3].copy_from_slice(&color);
    }

    pub fn get_pixels(&self) -> &[u8] {
        &self.vec_pixels
    }
}
