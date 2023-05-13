// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// layout nannou

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    pub config: Config,
    pub renderer: Renderer,
    pub width: i64,
    pub height: i64,
    pub buttons: Vec<Button>,
    pub sliders: Vec<Slider>,
    pub inputs: Vec<Input>,
    pub checkboxes: Vec<Checkbox>,
    pub texts: Vec<Text>,
    pub image: Vec<Image>,
}