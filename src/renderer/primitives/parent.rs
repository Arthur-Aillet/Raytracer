use crate::vector::Vector;
use serde::Serialize;

use crate::renderer::types::{Texture, Transform};

use super::{Intersection, Object};

#[derive(Serialize)]
pub struct Parent {
    pub name: String,
    pub obj_type: String,
    pub transform: Transform,
    pub children: Vec<Box<dyn Object + Send + Sync>>,
}

impl Object for Parent {
    fn apply_transform(&mut self) {}
    fn get_texture(&self) -> Texture {
        Texture::default()
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

        for object in self.children.iter() {
            if let Some(inters) = object.intersection(ray, origin) {
                let distance_found = (inters.intersection_point - origin).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }
        found_intersection
    }

    fn move_obj(&mut self, offset: Transform) {
        self.transform = self.transform + offset;
        for child in self.children.iter_mut() {
            child.move_obj(offset);
        }
    }

    fn set_texture(&mut self, _new: Texture) {}
    fn set_transform(&mut self, new: Transform) {
        self.transform = new
    }

    fn surface_position(&self, _position: Vector) -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_normal_map(&self) -> Texture {
        Texture::normal_map_default()
    }

    fn get_type(&self) -> String {
        self.obj_type.clone()
    }

    fn set_normal_map(&mut self, _new: Texture) {}
}
