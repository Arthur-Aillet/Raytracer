use crate::vector::Vector;
use serde::Serialize;

use crate::renderer::types::{Texture, Transform};

use super::{Intersection, Object};

#[derive(Serialize)]
pub struct Triangle {
    pub name: String,
    pub obj_type: String,
    pub transform: Transform,
    pub texture: Texture,
    pub normal_map: Texture,
    pub point_a: Vector,
    pub point_b: Vector,
    pub point_c: Vector,
    pub(crate) point_a_applied: Vector,
    pub(crate) point_b_applied: Vector,
    pub(crate) point_c_applied: Vector,
    pub(crate) normal: Vector,
    pub children: Vec<Box<dyn Object + Send + Sync>>,
}

impl Object for Triangle {
    fn apply_transform(&mut self) {
        self.point_a_applied = self.point_a;
        self.point_a_applied.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
        self.point_a_applied = self.point_a_applied + self.transform.pos;
        self.point_b_applied = self.point_b;
        self.point_b_applied.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
        self.point_b_applied = self.point_b_applied + self.transform.pos;
        self.point_c_applied = self.point_c;
        self.point_c_applied.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
        self.point_c_applied = self.point_c_applied + self.transform.pos;
        self.normal = (self.point_b_applied - self.point_a_applied)
            .cross_product(self.point_c_applied - self.point_a_applied)
            .normalize();
    }

    fn intersection(&self, ray: Vector, origin: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

        for object in self.children.iter() {
            let intersect = object.intersection(ray, origin);

            if let Some(inters) = intersect {
                let distance_found = (inters.intersection_point - origin).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }

        let denom = ray.normalize().dot_product(self.normal);
        if denom == 0.0 {
            return found_intersection;
        }
        let progress =
            (((self.point_a_applied + self.point_b_applied + self.point_c_applied) / 3.0) - origin)
                .dot_product(self.normal)
                / denom;
        if progress < 0.0 {
            return found_intersection;
        }
        let intersection_point = Vector {
            x: origin.x + ray.x * progress,
            y: origin.y + ray.y * progress,
            z: origin.z + ray.z * progress,
        };

        let cross = (self.point_b_applied - self.point_a_applied)
            .cross_product(intersection_point - self.point_a_applied);
        if self.normal.dot_product(cross) < 0.0 {
            return found_intersection;
        }

        let cross = (self.point_c_applied - self.point_b_applied)
            .cross_product(intersection_point - self.point_b_applied);
        if self.normal.dot_product(cross) < 0.0 {
            return found_intersection;
        }

        let cross = (self.point_a_applied - self.point_c_applied)
            .cross_product(intersection_point - self.point_c_applied);
        if self.normal.dot_product(cross) < 0.0 {
            return found_intersection;
        }
        if (intersection_point - origin).len() < smallest_distance {
            found_intersection = Some(Intersection {
                intersection_point,
                normal: self.normal,
                object: Some(self),
                light: None,
            })
        }
        found_intersection
    }

    fn surface_position(&self, position: Vector) -> Vector {
        let mut rotated_position = position.normalize();

        rotated_position.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
        Vector {
            x: (2.0
                * (1.0
                    - (rotated_position.x.atan2(rotated_position.y)
                        / (2.0 * std::f64::consts::PI)
                        + 0.5)))
                % 1.0,
            y: 1.0
                - (rotated_position.z
                    / (rotated_position.x.powi(2)
                        + rotated_position.y.powi(2)
                        + rotated_position.z.powi(2))
                    .sqrt())
                .acos()
                    / std::f64::consts::PI,
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

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> String {
        self.obj_type.clone()
    }

    fn set_transform(&mut self, new: Transform) {
        self.transform = new;
        self.apply_transform();
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
