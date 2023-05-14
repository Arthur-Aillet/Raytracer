// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// components for nannou layout

use nannou::color;
use nannou::prelude::*;

pub struct Button {
    pub name: String,
    pub rect: Rect,
    pub text: String,
}

pub struct Slider {
    pub name: String,
    pub rect: Rect,
    pub text: String,
    pub min: i64,
    pub max: i64,
    pub value: i64,
}

pub struct Input {
    pub name: String,
    pub rect: Rect,
    pub text: String,
    pub value: String,
}

pub struct Checkbox {
    pub name: String,
    pub rect: Rect,
    pub text: String,
    pub value: bool,
}

pub struct Text {
    pub name: String,
    pub rect: Rect,
    pub text: String,
}

pub struct Image {
    pub name: String,
    pub rect: Rect,
    pub path: String,
}

impl Button {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String) -> Button {
        Button {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
        }
    }
}

impl Slider {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String, value: i64, min: i64, max: i64) -> Slider {
        Slider {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
            min,
            max,
            value,
        }
    }
}

impl Input {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String, value: String) -> Input {
        Input {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
            value,
        }
    }
}

impl Checkbox {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String, value: bool) -> Checkbox {
        Checkbox {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
            value,
        }
    }
}

impl Text {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String) -> Text {
        Text {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
        }
    }
}

impl Image {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, path: String) -> Image {
        Image {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            path,
        }
    }
}
