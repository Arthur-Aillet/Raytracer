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
    pub window: RenderWindow
}

pub fn draw_buffer(config: &Config, window: &mut RenderWindow) {
    let mut texture = sfml::graphics::Texture::new().unwrap();
    let rect = sfml::graphics::IntRect::new(0, 0, config.width as i32, config.height as i32);
    let mut sprite = sfml::graphics::Sprite::new();

    texture.load_from_file(&config.save_file, rect).unwrap();
    sprite.set_texture(&texture, true);
    window.draw(&sprite);
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
            let renderer = Renderer::get_renderer_from_file(&self.config);

            PPMInterface::new(&self.config.save_file).write(self.config.width, self.config.height, renderer.unwrap().grender(&self.config, &mut self.window));
            self.window.clear(Color::BLACK);
            draw_buffer(&self.config, &mut self.window);
            self.window.display();
        }
    }
}
