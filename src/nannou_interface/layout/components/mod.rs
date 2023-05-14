// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// components for nannou layout

use nannou::color;
use nannou::prelude::*;

pub struct Button {
    pub name: String,
    pub rect: Rect,
    pub state: u8,
    pub text: String,
    pub color: LinSrgba<f32>,
    pub text_color: LinSrgba<f32>,
}

pub struct Slider {
    pub name: String,
    pub rect: Rect,
    pub text: String,
    pub color: LinSrgba<f32>,
    pub text_color: LinSrgba<f32>,
    pub value: f32,
}

pub struct Input {
    pub name: String,
    pub rect: Rect,
    pub text: String,
    pub color: LinSrgba<f32>,
    pub text_color: LinSrgba<f32>,
    pub value: String,
}

pub struct Checkbox {
    pub name: String,
    pub rect: Rect,
    pub text: String,
    pub color: LinSrgba<f32>,
    pub text_color: LinSrgba<f32>,
    pub value: bool,
}

pub struct Text {
    pub name: String,
    pub rect: Rect,
    pub text: String,
    pub color: LinSrgba<f32>,
    pub text_color: LinSrgba<f32>,
}

pub struct Image {
    pub name: String,
    pub rect: Rect,
    pub color: LinSrgba<f32>,
    pub text_color: LinSrgba<f32>,
    pub path: String,
}

impl Button {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String, color: LinSrgba<f32>, text_color: LinSrgba<f32>) -> Button {
        Button {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            state: 0,
            text,
            color,
            text_color,
        }
    }
}

impl Slider {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String, color: LinSrgba<f32>, text_color: LinSrgba<f32>, value: f32) -> Slider {
        Slider {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
            color,
            text_color,
            value,
        }
    }
}

impl Input {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String, color: LinSrgba<f32>, text_color: LinSrgba<f32>, value: String) -> Input {
        Input {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
            color,
            text_color,
            value,
        }
    }
}

impl Checkbox {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String, color: LinSrgba<f32>, text_color: LinSrgba<f32>, value: bool) -> Checkbox {
        Checkbox {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
            color,
            text_color,
            value,
        }
    }
}

impl Text {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String, color: LinSrgba<f32>, text_color: LinSrgba<f32>) -> Text {
        Text {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
            color,
            text_color,
        }
    }
}

impl Image {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, color: LinSrgba<f32>, text_color: LinSrgba<f32>, path: String) -> Image {
        Image {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            color,
            text_color,
            path,
        }
    }
}
