//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

pub mod cone;
pub mod cylinder;
pub mod mesh;
pub mod parent;
pub mod plane;
pub mod sphere;
pub mod triangle;

use super::types::{Texture, Transform};
use crate::renderer::lights::Light;
use crate::vector;
use erased_serde::serialize_trait_object;
use vector::Vector;

pub struct Intersection<'a> {
    pub intersection_point: Vector,
    pub normal: Vector,
    pub object: Option<&'a dyn Object>,
    pub light: Option<&'a dyn Light>,
}

pub trait Object: erased_serde::Serialize {
    fn apply_transform(&mut self);
    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection>;
    fn surface_position(&self, position: Vector) -> Vector;
    fn get_transform(&self) -> Transform;
    fn move_obj(&mut self, offset: Transform);
    fn set_transform(&mut self, new: Transform);
    fn get_name(&self) -> String;
    fn get_type(&self) -> String;
    fn get_texture(&self) -> Texture;
    fn set_texture(&mut self, new: Texture);
    fn get_normal_map(&self) -> Texture;
    fn set_normal_map(&mut self, new: Texture);
}

serialize_trait_object!(Object);
