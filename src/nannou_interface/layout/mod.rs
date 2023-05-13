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
            400.0,
            424.0,
            130.0,
            50.0,
            String::from("FAST"),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0x3F3944)),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::WHITE),
        ));
        buttons.push(Button::new(
            550.0,
            424.0,
            130.0,
            50.0,
            String::from("FANCY"),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0x3F3944)),
            nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::WHITE),
        ));
        sliders.push(Slider::new(
            400.0,
            500.0,
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
        for button in &self.buttons {
            draw.rect()
                .x_y(button.rect.x() + button.rect.w() / 2.0, button.rect.y() + button.rect.h() / 2.0)
                .w_h(button.rect.w(), button.rect.h())
                .color(button.color);

            draw.text(&button.text)
                .x_y(button.rect.x() + button.rect.w() / 2.0, button.rect.y() + button.rect.h() / 2.0)
                .color(button.text_color);
        }
    }

    fn display_sliders(&self, app: &App, model: &Model, frame: &Frame, draw: &Draw) {
        for slider in &self.sliders {
            draw.rect()
                .x_y(slider.rect.x() + slider.rect.w() / 2.0, slider.rect.y() + slider.rect.h() / 2.0)
                .w_h(slider.rect.w(), slider.rect.h())
                .color(slider.color);

            draw.text(&slider.text)
                .x_y(slider.rect.x() + slider.rect.w() / 2.0, slider.rect.y() + slider.rect.h() / 2.0)
                .color(slider.text_color);
        }
    }

    pub fn display(&self, app: &App, model: &Model, frame: &Frame, draw: &Draw) {
        self.display_buttons(app, model, frame, draw);
        self.display_sliders(app, model, frame, draw);
    }
}