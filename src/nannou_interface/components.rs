// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// components for nannou layout

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

pub struct Text {
    pub name: String,
    pub rect: Rect,
    pub text: String,
}

impl Button {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String) -> Button {
        Button {
            name,
            rect: Rect::from_x_y_w_h(x, y, w, h),
            text,
        }
    }

    pub fn display(&self, app: &App, draw: &Draw) {
        let mouse_position = app.mouse.position();

        if self.rect.contains(mouse_position) {
            if app.mouse.buttons.left().is_down() {
                draw.rect()
                    .x_y(self.rect.x(), self.rect.y())
                    .w_h(self.rect.w(), self.rect.h())
                    .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                        nannou::color::rgb_u32(0xA68F6D),
                    ));
            } else {
                draw.rect()
                    .x_y(self.rect.x(), self.rect.y())
                    .w_h(self.rect.w(), self.rect.h())
                    .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                        nannou::color::rgb_u32(0xA6701E),
                    ));
            }
        } else {
            draw.rect()
                .x_y(self.rect.x(), self.rect.y())
                .w_h(self.rect.w(), self.rect.h())
                .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                    nannou::color::rgb_u32(0xEB9E2C),
                ));
        }
        draw.text(&self.text)
            .x_y(self.rect.x(), self.rect.y())
            .w_h(self.rect.w(), self.rect.h())
            .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                nannou::color::rgb_u32(0xFFFFFF),
            ));
    }
}

impl Slider {
    pub fn new(
        name: String,
        dimensions: Rect,
        text: String,
        value: i64,
        min: i64,
        max: i64,
    ) -> Slider {
        Slider {
            name,
            rect: dimensions,
            text,
            min,
            max,
            value,
        }
    }

    pub fn display(&mut self, app: &App, draw: &Draw) {
        let mouse_position = app.mouse.position();
        let percent = (self.value - self.min) as f32 / (self.max - self.min) as f32;
        let mut cursor_rect = Rect::from_x_y_w_h(
            self.rect.x() - (self.rect.w() / 2.0) + (self.rect.w() * percent),
            self.rect.y(),
            self.rect.h() / 1.8,
            self.rect.h() / 1.8,
        );

        if cursor_rect.contains(mouse_position) {
            if app.mouse.buttons.left().is_down() {
                cursor_rect.x.start = mouse_position.x - (cursor_rect.w() / 2.0);
                cursor_rect.x.end = mouse_position.x + (cursor_rect.w() / 2.0);
            }
            if cursor_rect.x.start < self.rect.x() - (self.rect.w() / 2.0) {
                cursor_rect.x.start = self.rect.x() - (self.rect.w() / 2.0);
                cursor_rect.x.end = cursor_rect.x.start + cursor_rect.w();
            } else if cursor_rect.x.end > self.rect.x() + (self.rect.w() / 2.0) {
                cursor_rect.x.end = self.rect.x() + (self.rect.w() / 2.0);
                cursor_rect.x.start = cursor_rect.x.end - cursor_rect.w();
            }
        }

        //draw background
        draw.rect()
            .x_y(self.rect.x(), self.rect.y())
            .w_h(self.rect.w(), self.rect.h() / 2.2)
            .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                nannou::color::rgb_u32(0x3F3944),
            ));
        //draw curent value bar
        draw.rect()
            .x_y(
                (self.rect.x() - (self.rect.w() / 2.0)) + ((self.rect.w() * percent) / 2.0),
                self.rect.y(),
            )
            .w_h(self.rect.w() * percent, self.rect.h() / 2.2)
            .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                nannou::color::rgb_u32(0xA6701E),
            ));
        //draw cursor
        draw.rect()
            .x_y(cursor_rect.x(), cursor_rect.y())
            .w_h(cursor_rect.w(), cursor_rect.h())
            .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                nannou::color::rgb_u32(0xEB9E2C),
            ));

        draw.text(&self.text)
            .x_y(self.rect.x(), self.rect.y() + 25.0)
            .w_h(self.rect.w(), self.rect.h())
            .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                nannou::color::rgb_u32(0xFFFFFF),
            ));
        self.value = (((cursor_rect.x() - (self.rect.x() - (self.rect.w() / 2.0))) / self.rect.w())
            * (self.max - self.min) as f32
            + self.min as f32) as i64;
        draw.text(self.value.to_string().as_str())
            .x_y(
                self.rect.x() + (self.text.len() as f32 * 5.0),
                self.rect.y() + 25.0,
            )
            .w_h(self.rect.w(), self.rect.h())
            .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                nannou::color::rgb_u32(0xFFFFFF),
            ));
    }
}

impl Text {
    pub fn new(name: String, x: f32, y: f32, w: f32, h: f32, text: String) -> Text {
        let rect = Rect::from_x_y_w_h(x, y, w, h);

        Text { name, rect, text }
    }

    pub fn display(&mut self, _: &App, draw: &Draw) {
        draw.text(&self.text)
            .x_y(self.rect.x(), self.rect.y())
            .w_h(self.rect.w(), self.rect.h())
            .color(nannou::color::IntoLinSrgba::<f32>::into_lin_srgba(
                nannou::color::rgb_u32(0xFFFFFF),
            ));
    }
}
