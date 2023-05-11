// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// ppm interface module

use nannou::prelude::*;
use nannou::Frame;
use nannou::App;

use crate::renderer::Renderer;
use std::time::Duration;
use crate::config;
use crate::config::Config;

pub struct SfmlInterface {
    config: Config,
}