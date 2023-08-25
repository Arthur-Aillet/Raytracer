use crate::vector::Vector;
use serde::Serialize;

use crate::renderer::types::{Texture, Transform};

use super::{Intersection, Object};

#[derive(Serialize)]
pub struct Plane {
    pub name: String,
    pub obj_type: String,
    pub transform: Transform,
    pub texture: Texture,
    pub normal_map: Texture,
    pub normal: Vector,
    pub(crate) normal_applied: Vector,
    pub children: Vec<Box<dyn Object + Send + Sync>>,
}

impl Object for Plane {
    fn apply_transform(&mut self) {
        self.normal_applied = self.normal.normalize();
        self.normal_applied.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
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
        let denom = ray.normalize().dot_product(self.normal_applied);
        if denom == 0.0 {
            return found_intersection;
        }
        let progress = (self.transform.pos - origin).dot_product(self.normal_applied) / denom;
        if progress < 0.0 {
            return found_intersection;
        }
        let intersection_point = Vector {
            x: origin.x + ray.x * progress,
            y: origin.y + ray.y * progress,
            z: origin.z + ray.z * progress,
        };
        if (intersection_point - origin).len() < smallest_distance {
            found_intersection = Some(Intersection {
                intersection_point,
                normal: self.normal_applied,
                object: Some(self),
                light: None,
            })
        }
        found_intersection
    }

    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position;

        rotated_position.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
        Vector {
            x: (position.x + 100.0) % 1.0,
            y: (position.y + 100.0) % 1.0,
            z: 0.0,
        }
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn move_obj(&mut self, offset: Transform) {
        self.transform = self.transform + offset;
        for child in self.children.iter_mut() {
            child.move_obj(offset);
        }
        self.apply_transform();
    }

    fn set_transform(&mut self, new: Transform) {
        self.transform = new;
        self.apply_transform();
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> String {
        self.obj_type.clone()
    }

    fn get_texture(&self) -> Texture {
        self.texture.clone()
    }

    fn set_texture(&mut self, new: Texture) {
        self.texture = new
    }

    fn get_normal_map(&self) -> Texture {
        self.normal_map.clone()
    }

    fn set_normal_map(&mut self, new: Texture) {
        self.normal_map = new
    }
}
