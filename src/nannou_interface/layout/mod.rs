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
            rect: Rect::from_x_y_w_h(width, height, 360.0, config.height as f32),
            buttons,
            sliders,
            inputs,
            checkboxes,
            texts,
            image,
        }
    }

    pub fn refresh_objects(&mut self, app: &App, draw: &Draw, renderer: &Renderer) {
        let mut nb_objects_futur = renderer.primitives.len();
        let mut nb_objects = self.texts.len();
        let mut count = 1;
        let mut text_name = String::new();

        if nb_objects_futur != nb_objects {
            let mut y = (self.config.height as f32 / 2.0) - 175.0;

            self.texts.clear();
            self.texts.push(Text::new("object infos".to_string(), (self.config.width as f32 / 2.0) - 310.0, y + 25.0, 280.0, 50.0, String::from("OBJECTS :")));
            for object in &renderer.primitives {
                if (count == nb_objects_futur) {
                    text_name = format!("└── {} : {}", object.get_type(), object.get_name());
                } else {
                    text_name = format!("├── {} : {}", object.get_type(), object.get_name());
                }
                let text = Text::new(format!("└── {} : {}", object.get_type(), object.get_name()), (self.config.width as f32 / 2.0) - 300.0 + (object.get_name().len() as f32 * 3.5), y, 280.0, 50.0, text_name.clone());
                self.texts.push(text);
                y -= 25.0;
                count += 1;
            }
        }
    }


    pub fn display(&mut self, app: &App, draw: &Draw, renderer: &Renderer) {
        self.refresh_objects(app, draw, renderer);

        for button in &mut self.buttons {
            button.display(app, draw);
        }
        for slider in &mut self.sliders {
            slider.display(app, draw);
        }
        for input in &mut self.inputs {
            input.display(app, draw);
        }
        for checkbox in &mut self.checkboxes {
            checkbox.display(app, draw);
        }
        for text in &mut self.texts {
            text.display(app, draw);
        }
        for image in &mut self.image {
            image.display(app, draw);
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
