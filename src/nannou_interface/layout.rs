// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// layout nannou

use nannou::prelude::*;

use crate::config::Config;
use crate::nannou_interface::components::{Button, Slider, Text};
use crate::renderer::Renderer;

pub struct Layout {
    pub config: Config,
    pub renderer: Renderer,
    pub rect: Rect,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
    pub texts: Vec<Text>,
}

impl Layout {
    pub fn new(config: Config) -> Layout {
        let renderer = Renderer::get_renderer_from_file(&config).unwrap();
        let height = config.height as f32 / 2.0;
        let width = (config.width as f32 + 360.0) / 2.0;

        let buttons = vec![
            Button::new(
                "fast".to_string(),
                width - 260.0,
                height - 50.0,
                120.0,
                50.0,
                String::from("FAST"),
            ),
            Button::new(
                "fancy".to_string(),
                width - 100.0,
                height - 50.0,
                120.0,
                50.0,
                String::from("FANCY"),
            ),
            Button::new(
                "exit".to_string(),
                width - 180.0,
                -height + 50.0,
                280.0,
                50.0,
                String::from("EXIT"),
            ),
        ];
        let sliders = vec![Slider::new(
            "fov".to_string(),
            Rect::from_x_y_w_h(width - 180.0, height - 120.0, 280.0, 50.0),
            String::from("FOV : "),
            renderer.camera.fov,
            0,
            180,
        )];
        let texts = vec![Text::new(
            "object info".to_string(),
            width - 180.0,
            height - 120.0,
            280.0,
            50.0,
            String::from("Obj infos : "),
        )];

        Layout {
            config: config.clone(),
            renderer,
            rect: Rect::from_x_y_w_h(width, height, 360.0, config.height as f32),
            buttons,
            sliders,
            texts,
        }
    }

    pub fn refresh_objects(&mut self, renderer: &Renderer) {
        let nb_objects_futur = renderer.primitives.len();
        let nb_objects = self.texts.len();
        let mut count = 1;
        let mut text_name;

        if nb_objects_futur != nb_objects {
            let mut y = (self.config.height as f32 / 2.0) - 175.0;

            self.texts.clear();
            self.texts.push(Text::new(
                "object infos".to_string(),
                ((self.config.width as f32 + self.rect.w()) / 2.0) - 310.0,
                y + 25.0,
                280.0,
                50.0,
                String::from("OBJECTS :"),
            ));

            for object in &renderer.primitives {
                if count == nb_objects_futur {
                    text_name = format!("└── {} : {}", object.get_type(), object.get_name());
                } else {
                    text_name = format!("├── {} : {}", object.get_type(), object.get_name());
                }
                self.texts.push(Text::new(
                    format!("└── {} : {}", object.get_type(), object.get_name()),
                    ((self.config.width as f32 + self.rect.w()) / 2.0) - 300.0
                        + (object.get_name().len() as f32 * 3.5),
                    y,
                    280.0,
                    50.0,
                    text_name.clone(),
                ));
                y -= 25.0;
                count += 1;
            }
        }
    }

    pub fn display(&mut self, app: &App, draw: &Draw, renderer: &Renderer) {
        self.refresh_objects(renderer);

        for button in &mut self.buttons {
            button.display(app, draw);
        }
        for slider in &mut self.sliders {
            slider.display(app, draw);
        }
        for text in &mut self.texts {
            text.display(app, draw);
        }
    }

    pub fn get_buttons_interactions(&self, app: &App, name: String) -> bool {
        for button in &self.buttons {
            if button.name == name
                && button.rect.contains(app.mouse.position())
                && app.mouse.buttons.left().is_down()
            {
                return true;
            }
        }
        false
    }

    pub fn get_sliders_interactions(&self, name: String) -> i64 {
        for slider in &self.sliders {
            if slider.name == name {
                return slider.value;
            }
        }
        -1
    }
}
