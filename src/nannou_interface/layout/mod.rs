// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// layout nannou

mod components;

use nannou::prelude::*;

use crate::config::Config;
use crate::renderer::Renderer;
use crate::nannou_interface::layout::components::{Button, Slider, Input, Checkbox, Text, Image};
use crate::nannou_interface::Model;

#[derive(PartialEq)]
pub enum ComponentType {
    Button,
    Slider,
    Input,
    Checkbox,
    Text,
    Image,
}

pub struct Layout {
    pub config: Config,
    pub renderer: Renderer,
    pub rect: Rect,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
    pub inputs: Vec<Input>,
    pub checkboxes: Vec<Checkbox>,
    pub texts: Vec<Text>,
    pub image: Vec<Image>,
}

impl Layout {
    pub fn new(config: Config) -> Layout {
        let renderer = Renderer::get_renderer_from_file(&config).unwrap();
        let height = config.height as f32 / 2.0;
        let width = (config.width as f32 + 360.0) / 2.0;

        let mut inputs = Vec::new();
        let mut checkboxes = Vec::new();
        let mut image = Vec::new();

        let mut buttons = vec![
            Button::new("fast".to_string(), width - 260.0, height - 50.0, 120.0, 50.0, String::from("FAST")),
            Button::new("fancy".to_string(), width - 100.0, height - 50.0, 120.0, 50.0, String::from("FANCY")),
            Button::new("exit".to_string(), width - 180.0, -height + 50.0, 280.0, 50.0, String::from("EXIT")),
        ];
        let mut sliders = vec![
            Slider::new("fov".to_string(), width - 180.0, height - 120.0, 280.0, 50.0, String::from("FOV : "), renderer.camera.fov, 0, 180),
        ];
        let mut texts = vec![
            Text::new("object info".to_string(), width - 180.0, height - 120.0, 280.0, 50.0, String::from("Obj infos : ")),
        ];

        Layout {
            config: config.clone(),
            renderer: renderer,
            rect: Rect::from_x_y_w_h(400.0, 600.0, 360.0, config.height as f32),
            buttons,
            sliders,
            inputs,
            checkboxes,
            texts,
            image,
        }
    }

    pub fn display(&mut self, app: &App, draw: &Draw) {
        for button in &mut self.buttons {
            button.display(app, draw);
        }
        for slider in &mut self.sliders {
            slider.display(app, draw);
        }
    }

    pub fn get_buttons_interactions(&self, app: &App, name: String) -> bool {
        for button in &self.buttons {
            if button.name == name {
                if button.rect.contains(app.mouse.position()) {
                    if app.mouse.buttons.left().is_down() {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn get_sliders_interactions(&self, app: &App, name: String) -> i64 {
        for slider in &self.sliders {
            if slider.name == name {
                return slider.value;
            }
        }
        return -1;
    }
}
