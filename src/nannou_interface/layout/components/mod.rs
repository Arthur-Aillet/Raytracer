// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// components for nannou layout

#[derive(Debug, Clone, Copy)]
pub struct Button {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub text: String,
    pub color: [f32; 4],
    pub text_color: [f32; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct Slider {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub text: String,
    pub color: [f32; 4],
    pub text_color: [f32; 4],
    pub value: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub text: String,
    pub color: [f32; 4],
    pub text_color: [f32; 4],
    pub value: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Checkbox {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub text: String,
    pub color: [f32; 4],
    pub text_color: [f32; 4],
    pub value: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Text {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub text: String,
    pub color: [f32; 4],
    pub text_color: [f32; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct Image {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub color: [f32; 4],
    pub text_color: [f32; 4],
    pub path: String,
}
