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
        let mut texts = Vec::new();
        let mut image = Vec::new();

        let mut buttons = vec![
            Button::new("fast".to_string(), width - 260.0, height - 50.0, 120.0, 50.0, String::from("FAST")),
            Button::new("fancy".to_string(), width - 100.0, height - 50.0, 120.0, 50.0, String::from("FANCY")),
        ];
        let mut sliders = vec![
            Slider::new("fov".to_string(), width - 180.0, height - 120.0, 280.0, 50.0, String::from("FOV"), renderer.camera.fov, 0, 180),
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

    fn display_buttons(&mut self, app: &App, draw: &Draw) {
        let mouse_position = app.mouse.position();

        for button in &mut self.buttons {
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

    pub fn display_sliders(&mut self, app: &App, draw: &Draw) {
        let mouse_position = app.mouse.position();

        for slider in &mut self.sliders {
            let percent = (slider.value - slider.min) as f32 / (slider.max - slider.min) as f32;
            let mut cursor_rect = Rect::from_x_y_w_h(
                slider.rect.x() - (slider.rect.w() / 2.0) + (slider.rect.w() * percent as f32),
                slider.rect.y(),
                slider.rect.h() / 1.8,
                slider.rect.h() / 1.8,
            );

            if cursor_rect.contains(mouse_position) {
                if app.mouse.buttons.left().is_down() {
                    cursor_rect.x.start = mouse_position.x - (cursor_rect.w() / 2.0);
                    cursor_rect.x.end = mouse_position.x + (cursor_rect.w() / 2.0);
                }
                // limit cursor to slider
                if cursor_rect.x.start < slider.rect.x() - (slider.rect.w() / 2.0) {
                    cursor_rect.x.start = slider.rect.x() - (slider.rect.w() / 2.0);
                    cursor_rect.x.end = cursor_rect.x.start + cursor_rect.w();
                } else if cursor_rect.x.end > slider.rect.x() + (slider.rect.w() / 2.0) {
                    cursor_rect.x.end = slider.rect.x() + (slider.rect.w() / 2.0);
                    cursor_rect.x.start = cursor_rect.x.end - cursor_rect.w();
                }
            }

            //draw background
            draw.rect()
                .x_y(slider.rect.x(), slider.rect.y())
                .w_h(slider.rect.w(), slider.rect.h() / 2.2)
                .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0x3F3944)));
            //draw curent value bar
            draw.rect()
                .x_y((slider.rect.x() - (slider.rect.w() / 2.0)) + ((slider.rect.w() * percent as f32) / 2.0), slider.rect.y())
                .w_h(slider.rect.w() * percent, slider.rect.h() / 2.2)
                .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xA6701E)));
            //draw cursor
            draw.rect()
                .x_y(cursor_rect.x(), cursor_rect.y())
                .w_h(cursor_rect.w(), cursor_rect.h())
                .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xEB9E2C)));

            draw.text(&slider.text)
                .x_y(slider.rect.x(), slider.rect.y() + 25.0)
                .w_h(slider.rect.w(), slider.rect.h())
                .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(nannou::color::rgb_u32(0xFFFFFF)));
            slider.value = (((cursor_rect.x() - (slider.rect.x() - (slider.rect.w() / 2.0))) as f32 / slider.rect.w()) as f32 * (slider.max - slider.min) as f32 + slider.min as f32) as i64;
        }
    }

    pub fn display(&mut self, app: &App, draw: &Draw) {
        self.display_buttons(app, draw);
        self.display_sliders(app, draw);
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
