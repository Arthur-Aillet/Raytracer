use crate::vector::{resolve_quadratic_equation, Vector};
use serde::Serialize;

use crate::renderer::types::{Texture, Transform};

use super::{Intersection, Object};

#[derive(Serialize)]
pub struct Sphere {
    pub name: String,
    pub obj_type: String,
    pub transform: Transform,
    pub texture: Texture,
    pub normal_map: Texture,
    pub radius: f64,
    pub(crate) radius_applied: f64,
    pub children: Vec<Box<dyn Object + Send + Sync>>,
}

impl Object for Sphere {
    fn apply_transform(&mut self) {
        self.radius_applied = self.radius * self.transform.scale;
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
        let diff = origin - self.transform.pos;
        let result = resolve_quadratic_equation(
            ray.dot_product(ray), // could be 1 if normalized
            2.0 * (ray.dot_product(diff)),
            (diff.dot_product(diff)) - self.radius_applied.powi(2),
        );

        let smallest_result: Option<&f64> = result
            .iter()
            .filter(|number| **number > 0.0)
            .min_by(|a, b| a.partial_cmp(b).unwrap());

        if let Some(smallest) = smallest_result {
            let point = Vector {
                x: origin.x + ray.x * smallest,
                y: origin.y + ray.y * smallest,
                z: origin.z + ray.z * smallest,
            };

            if (point - origin).len() < smallest_distance {
                return Some(Intersection {
                    normal: point - self.transform.pos,
                    intersection_point: point,
                    object: Some(self),
                    light: None,
                });
            }
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
