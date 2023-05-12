// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module

use crate::config::Config;
use sfml::graphics::{RenderWindow, RenderTarget, Color, Image};
use sfml::window::{Event, Style};
use crate::renderer::Renderer;
use crate::ppm_interface::PPMInterface;

pub struct SfmlInterface {
    config: Config,
    window: RenderWindow
}

impl SfmlInterface {
    pub fn new(config: Config) -> Self {
        let mut window = RenderWindow::new(
            (config.width as u32, config.height as u32),
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
            self.window.clear(Color::BLACK);
            self.draw_buffer();
            self.window.display();
        }
    }

    fn draw_buffer(&mut self) {
        let renderer = Renderer::get_renderer_from_file(&self.config);
        PPMInterface::new(&self.config.save_file).write(self.config.width, self.config.height, renderer.unwrap().pull_new_image(&self.config));

        let mut texture = sfml::graphics::Texture::new().unwrap();
        let rect = sfml::graphics::IntRect::new(0, 0, self.config.width as i32, self.config.height as i32);
        let mut sprite = sfml::graphics::Sprite::new();

        texture.load_from_file("scene_example.ppm", rect).unwrap();
        sprite.set_texture(&texture, true);
        self.window.draw(&sprite);
    }
}
