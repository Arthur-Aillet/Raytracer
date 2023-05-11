// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module

use nannou::prelude::*;
use nannou::Frame;
use nannou::App;

use crate::renderer::Renderer;
use std::time::Duration;
use crate::config;
use crate::config::Config;

pub struct SfmlInterface {
    config: Config,
    window: RenderWindow
}

impl SfmlInterface {
    pub fn new(mut config: &Config) -> Self {
        let mut window = RenderWindow::new(
            (config.width, config.height),
            "Rustracer",
            Style::CLOSE,
            &Default::default(),
        );
        window.set_vertical_sync_enabled(true);
        window.set_framerate_limit(60);


        SfmlInterface {
            config,
            window
        }
    }

    pub fn run(&mut self) {
        while self.window.is_open() {
            while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed => self.window.close(),
                    _ => {}
                }
            }
            self.window.clear(&Color::BLACK);
            self.window.display();
        }
    }
}