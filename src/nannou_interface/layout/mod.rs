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
        let mut buttons = Vec::new();
        let mut sliders = Vec::new();
        let mut inputs = Vec::new();
        let mut checkboxes = Vec::new();
        let mut texts = Vec::new();
        let mut image = Vec::new();

        buttons.push(Button::new(
            "fast".to_string(),
            465.0,
            450.0,
            130.0,
            50.0,
            String::from("FAST"),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xEB9E2C)),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::WHITE),
        ));
        buttons.push(Button::new(
            "fancy".to_string(),
            615.0,
            450.0,
            130.0,
            50.0,
            String::from("FANCY"),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xEB9E2C)),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::WHITE),
        ));
        sliders.push(Slider::new(
            "fov".to_string(),
            400.0,
            300.0,
            280.0,
            50.0,
            String::from("FOV"),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0x3F3944)),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::WHITE),
            0.0,
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
        // get the mouse position
        let mouse_position = app.mouse.position();

        // change the color of the button if the mouse is on it or it is clicked
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
                .color(button.text_color);
        }
    }

    pub fn display(&self, app: &App, model: &Model, frame: &Frame, draw: &Draw) {
        self.display_buttons(app, model, frame, draw);
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
