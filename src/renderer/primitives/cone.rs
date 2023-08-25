use crate::vector::{resolve_quadratic_equation, Vector};
use serde::Serialize;

use crate::renderer::types::{Texture, Transform};

use super::{Intersection, Object};

#[derive(Serialize)]
pub struct Cone {
    pub name: String,
    pub obj_type: String,
    pub transform: Transform,
    pub texture: Texture,
    pub normal_map: Texture,
    pub radius: f64,
    pub height: f64,
    pub(crate) axis: Vector,
    pub(crate) top: Vector,
    pub(crate) base: Vector,
    pub(crate) radius_applied: f64,
    pub(crate) height_applied: f64,
    pub children: Vec<Box<dyn Object + Send + Sync>>,
}

impl Cone {
    fn base_intersection(
        &self,
        ray: Vector,
        origin: Vector,
        normal: Vector,
        center: Vector,
    ) -> Option<Intersection> {
        let denom = ray.normalize().dot_product(normal);
        if denom == 0.0 {
            return None;
        }
        let progress = (center - origin).dot_product(normal) / denom;
        if progress < 0.0 {
            return None;
        }
        let intersection_point = Vector {
            x: origin.x + ray.x * progress,
            y: origin.y + ray.y * progress,
            z: origin.z + ray.z * progress,
        };

        if (intersection_point - center).len() > self.radius_applied {
            return None;
        }
        Some(Intersection {
            intersection_point,
            normal,
            object: Some(self),
            light: None,
        })
    }
}

impl Object for Cone {
    fn apply_transform(&mut self) {
        self.axis = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        self.axis.rotate(
            self.transform.rotation.x,
            self.transform.rotation.y,
            self.transform.rotation.z,
        );
        self.base = self.transform.pos - self.axis * (self.height / 2.0);
        self.top = self.transform.pos + self.axis * (self.height / 2.0);
        self.radius_applied = self.radius * self.transform.scale;
        self.height_applied = self.height * self.transform.scale;
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

        let distance = origin - self.top; // W

        let radius_constant = self.radius_applied.powi(2) / self.height_applied.powi(2);

        let a = 1.0
            - radius_constant * (ray.dot_product(self.axis)).powi(2)
            - (ray.dot_product(self.axis)).powi(2);
        let b = 2.0
            * (ray.dot_product(distance)
                - radius_constant * ray.dot_product(self.axis) * distance.dot_product(self.axis)
                - ray.dot_product(self.axis) * distance.dot_product(self.axis));
        let c = distance.dot_product(distance)
            - radius_constant * distance.dot_product(self.axis).powi(2)
            - distance.dot_product(self.axis).powi(2);

        let result = resolve_quadratic_equation(a, b, c);

        let smallest_result: Option<&f64> = result
            .iter()
            .filter(|number| **number > 0.0)
            .min_by(|fst, snd| fst.partial_cmp(snd).unwrap());
        smallest_result?;

        let intersection_point = origin + ray * *smallest_result.unwrap();
        if 0.0 <= (intersection_point - self.base).dot_product(self.axis)
            && (intersection_point - self.base).dot_product(self.axis) <= self.height_applied
        {
            // too far from center*/
            let cos_angle = self.axis.dot_product(self.top - intersection_point);
            let normal = (intersection_point
                - (self.top - self.axis * ((self.top - intersection_point).len2() / cos_angle)))
                .normalize();
            if (intersection_point - origin).len() < smallest_distance {
                return Some(Intersection {
                    intersection_point,
                    normal,
                    object: Some(self),
                    light: None,
                });
            } else {
                return found_intersection;
            }
        }
        let base = self.base_intersection(ray, origin, self.axis * -1.0, self.base);
        if (intersection_point - self.top).dot_product(self.axis) < 0.0 {
            if base.is_some()
                && (base.as_ref().unwrap().intersection_point - origin).len() < smallest_distance
            {
                return base;
            } else {
                return found_intersection;
            }
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
            x: 1.0
                - (rotated_position.x.atan2(rotated_position.y) / (2.0 * std::f64::consts::PI)
                    + 0.5),
            y: rotated_position.z % 1.0,
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
