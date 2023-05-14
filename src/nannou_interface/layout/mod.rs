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
    pub fn new(config: Config, renderer: Renderer) -> Layout {
        let height = config.height as f32 / 2.0;
        let width = (config.width as f32 + 360.0) / 2.0;

        let mut buttons = Vec::new();
        let mut sliders = Vec::new();
        let mut inputs = Vec::new();
        let mut checkboxes = Vec::new();
        let mut texts = Vec::new();
        let mut image = Vec::new();

        buttons.push(Button::new(
            "fast".to_string(),
            width - 260.0,
            height - 75.0,
            120.0,
            50.0,
            String::from("FAST"),
        ));
        buttons.push(Button::new(
            "fancy".to_string(),
            width - 100.0,
            height - 75.0,
            120.0,
            50.0,
            String::from("FANCY"),
        ));
        sliders.push(Slider::new(
            "fov".to_string(),
            width - 180.0,
            height - 150.0,
            280.0,
            50.0,
            String::from("FOV"),
            renderer.camera.fov,
            0,
            180,
        ));

        Layout {
            config: config.clone(),
            renderer,
            rect: Rect::from_x_y_w_h(400.0, 600.0, 360.0, config.height as f32),
            buttons,
            sliders,
            inputs,
            checkboxes,
            texts,
            image,
        }
    }

    fn display_buttons(&self, app: &App, model: &Model, frame: &Frame, draw: &Draw) {
        let mouse_position = app.mouse.position();

        for button in &self.buttons {
            if button.rect.contains(mouse_position) {
                if app.mouse.buttons.left().is_down() {
                    draw.rect()
                        .x_y(button.rect.x(), button.rect.y())
                        .w_h(button.rect.w(), button.rect.h())
                        .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xA68F6D)));
                } else {
                    draw.rect()
                        .x_y(button.rect.x(), button.rect.y())
                        .w_h(button.rect.w(), button.rect.h())
                        .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xA6701E)));
                }
            } else {
                draw.rect()
                    .x_y(button.rect.x(), button.rect.y())
                    .w_h(button.rect.w(), button.rect.h())
                    .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xEB9E2C)));
            }
            draw.text(&button.text)
                .x_y(button.rect.x(), button.rect.y())
                .w_h(button.rect.w(), button.rect.h())
                .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xFFFFFF)));
        }
    }

    pub fn display_sliders(&self, app: &App, model: &Model, frame: &Frame, draw: &Draw) {
        for slider in &self.sliders {
            draw.rect()
                .x_y(slider.rect.x(), slider.rect.y())
                .w_h(slider.rect.w(), slider.rect.h())
                .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0x23A5F9)));
        }
    }

    pub fn display(&self, app: &App, model: &Model, frame: &Frame, draw: &Draw) {
        self.display_buttons(app, model, frame, draw);
        self.display_sliders(app, model, frame, draw);
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

    pub fn get_interactions(&self, app: &App, name: String, component_type: ComponentType) -> bool {
        if component_type == ComponentType::Button {
            if self.get_buttons_interactions(app, name) == true {
                return true
            }
        }
        return false
    }
}
