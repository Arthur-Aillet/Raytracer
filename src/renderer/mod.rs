//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer common structures
//

mod camera;
mod lights;
mod parsing;
mod primitives;
pub mod types;

use crate::config::Config;
use crate::renderer::primitives::{Intersection, Object};
use crate::vector::Vector;
use camera::Camera;
use lights::Lights;
use parsing::Parser;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

use self::types::Texture;

pub struct Renderer {
    pub camera: Camera,
    pub primitives: Vec<Box<dyn Object + Send + Sync>>,
    pub lights: Lights,
    pub skybox: Texture,
}

struct Recursivity {
    general: i64,
    transmission: i64,
}

impl Renderer {
    fn found_nearest_intersection_fast(&self, origin: Vector, ray: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

        for object in self.primitives.iter() {
            let intersect = object.intersection(ray, origin);

            if let Some(inters) = intersect {
                let distance_found = (inters.intersection_point - origin).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }
        found_intersection
    }

    fn found_nearest_intersection(&self, origin: Vector, ray: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

        for object in self.primitives.iter() {
            let intersect = object.intersection(ray, origin);

            if let Some(inters) = intersect {
                let distance_found = (inters.intersection_point - origin).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }
        for light in self.lights.lights.iter() {
            if light.get_visible() {
                continue;
            }
            let intersect = light.intersection(ray, origin);

            if let Some(inters) = intersect {
                let distance_found = (inters.intersection_point - origin).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }
        found_intersection
    }

    fn get_ambient(&self, object: &dyn Object, position: Vector) -> Vector {
        let mut self_color = Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        for ambient in self.lights.ambient.iter() {
            let texture_coordinates =
                object.surface_position(position - object.get_transform().pos);
            self_color = self_color
                + object
                    .get_texture()
                    .texture(texture_coordinates.x, texture_coordinates.y)
                    .as_vector()
                    * object.get_texture().ambient
                    * ambient.color.as_vector()
                    * ambient.strength
                    * self.camera.ambient;
        }
        self_color
    }

    fn combine_pixel(&self, samples: &[Vector]) -> Vector {
        let mut result: Vector = samples[0];

        for (index, &sample) in samples.iter().enumerate().skip(1) {
            result = ((result * (index - 1) as f64) + (sample)) / index as f64;
        }
        result
    }

    fn skybox_position(&self, position: Vector) -> Vector {
        Vector {
            x: (2.0 * (1.0 - (position.x.atan2(position.y) / (2.0 * std::f64::consts::PI) + 0.5)))
                % 1.0,
            y: 1.0
                - (position.z
                    / (position.x.powi(2) + position.y.powi(2) + position.z.powi(2)).sqrt())
                .acos()
                    / std::f64::consts::PI,
            z: 0.0,
        }
    }

    fn get_color_from_ray_fast(&self, origin: Vector, ray: Vector) -> Vector {
        let maybe_intersect = self.found_nearest_intersection_fast(origin, ray);

        if let Some(intersect) = maybe_intersect {
            let normal_vector = intersect.normal.normalize();
            let light_vector =
                (self.camera.transform.pos - intersect.intersection_point).normalize();

            let texture_coordinates = intersect.object.unwrap().surface_position(
                intersect.intersection_point - intersect.object.unwrap().get_transform().pos,
            );
            let ambient = intersect
                .object
                .unwrap()
                .get_texture()
                .texture(texture_coordinates.x, texture_coordinates.y)
                .as_vector()
                * intersect.object.unwrap().get_texture().ambient
                * self.camera.ambient;

            let diffuse = light_vector.dot_product(normal_vector).max(0.0)
                * self.camera.diffuse
                * intersect.object.unwrap().get_texture().diffuse;

            ambient
                + intersect
                    .object
                    .unwrap()
                    .get_texture()
                    .texture(texture_coordinates.x, texture_coordinates.y)
                    .as_vector()
                    * diffuse
        } else {
            let skybox_position = self.skybox_position(ray);
            self.skybox
                .texture(skybox_position.x, skybox_position.y)
                .as_vector()
        }
    }

    fn refract(&self, normal: Vector, incident: Vector, ior1: f64, ior2: f64) -> Option<Vector> {
        let ratio = ior1 / ior2;
        let cos_i = normal.normalize().dot_product(incident.normalize()) * -1.0;
        let sin_t2 = ratio * ratio * (1.0 - cos_i * cos_i);
        if sin_t2 > 1.0 {
            return None;
        }
        let cos_t = (1.0 - sin_t2).sqrt();
        let final_vect = (incident.normalize() * ratio
            + normal.normalize() * (ratio * cos_i - cos_t))
            .normalize();
        Some(final_vect)
    }

    fn transmission(
        &self,
        intersect: &Intersection,
        incident_ray: Vector,
        recursivity: &mut Recursivity,
    ) -> Vector {
        let normal = intersect.normal.normalize();
        let other_ior = 1.0;
        let object_ior = intersect.object.unwrap().get_texture().ior;

        let maybe_new_ray = if recursivity.transmission <= 1 {
            self.refract(
                normal.normalize() * -1.0,
                incident_ray.normalize(),
                object_ior,
                other_ior,
            )
        } else {
            self.refract(
                normal.normalize(),
                incident_ray.normalize(),
                other_ior,
                object_ior,
            )
        };
        if let Some(new_ray) = maybe_new_ray {
            let maybe_intersect = self.found_nearest_intersection(
                intersect.intersection_point + new_ray * self.camera.shadow_bias,
                new_ray,
            );
            if let Some(new_intersect) = maybe_intersect {
                if recursivity.transmission == 2
                    && new_intersect.object.unwrap().get_texture().transmission > 0.0
                {
                    recursivity.transmission = 1;
                    return self.transmission(&new_intersect, new_ray, recursivity);
                } else {
                    recursivity.general -= 1;
                    return self.get_color_from_ray(
                        intersect.intersection_point + new_ray * self.camera.shadow_bias,
                        new_ray,
                        recursivity,
                    );
                }
            }
        } else {
            return Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn get_color_from_ray(
        &self,
        origin: Vector,
        ray: Vector,
        recursivity: &mut Recursivity,
    ) -> Vector {
        if recursivity.general == 0 {
            return Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        let maybe_intersect = self.found_nearest_intersection(origin, ray);

        if let Some(intersect) = maybe_intersect {
            if self.camera.display_normals {
                return intersect.normal.normalize() * 0.5 + 0.5;
            } else if self.camera.display_location {
                return intersect.intersection_point * 0.5 + 0.5;
            } else if self.camera.display_dot_product {
                return Vector {
                    x: intersect.normal.normalize().dot_product(ray.normalize()) * 0.5 + 0.5,
                    y: intersect.normal.normalize().dot_product(ray.normalize()) * 0.5 + 0.5,
                    z: intersect.normal.normalize().dot_product(ray.normalize()) * 0.5 + 0.5,
                };
            }

            // case of direct intersection with light object
            if let Some(light_touched) = intersect.light {
                return light_touched.get_color().as_vector();
            }
            let mut self_color =
                self.get_ambient(intersect.object.unwrap(), intersect.intersection_point);

            // calculation of lighting
            for light in self.lights.lights.iter() {
                self_color = self_color
                    + light.calculate_light(&intersect, ray, self.camera, &self.primitives);
            }

            let surface_point =
                intersect.intersection_point + intersect.normal * self.camera.shadow_bias;

            self_color = self_color * (1.0 - intersect.object.unwrap().get_texture().metalness);
            if recursivity.general <= 1 {
                return self_color;
            }
            let samples_nbr = (1.0
                + self.camera.reflection_samples as f64
                    * intersect.object.unwrap().get_texture().roughness)
                .powf(intersect.object.unwrap().get_texture().sampling_ponderation);
            for _ in 0..samples_nbr as i32 {
                let mut rng = rand::thread_rng();
                // random vector used for the roughness
                let random_a: f64 = rng.gen_range(0.0..std::f64::consts::TAU);
                let random_b: f64 = rng.gen_range(0.0..std::f64::consts::TAU);
                let random_vect = Vector {
                    x: random_a.cos() * random_b.cos(),
                    y: random_a.sin() * random_b.cos(),
                    z: random_b.sin(),
                };
                let mut reflection_ray = (ray.normalize()
                    - (intersect.normal.normalize()
                        * 2.0
                        * intersect.normal.normalize().dot_product(ray.normalize())))
                .normalize();
                if intersect.object.unwrap().get_texture().roughness != 0.0 {
                    reflection_ray.lerp(
                        &random_vect,
                        intersect.object.unwrap().get_texture().roughness,
                    );
                }
                let metalness = intersect.object.unwrap().get_texture().metalness;
                let new_color;
                if intersect.object.unwrap().get_texture().transmission == 0.0 {
                    recursivity.general -= 1;
                    new_color = self.get_color_from_ray(surface_point, reflection_ray, recursivity);
                    self_color = self_color
                        + ((new_color
                            * (1.0 - metalness)
                            * intersect.object.unwrap().get_texture().specular)
                            + (new_color
                                * intersect.object.unwrap().get_texture().color.as_vector()
                                * metalness))
                            * (1.0 / samples_nbr);
                } else {
                    recursivity.transmission = 2;
                    self_color = self.transmission(&intersect, ray, recursivity);
                }
            }
            if intersect.object.unwrap().get_texture().alpha != 1.0 {
                recursivity.general -= 1;
                let new_color = self.get_color_from_ray(
                    intersect.intersection_point + ray * self.camera.shadow_bias,
                    ray,
                    recursivity,
                );
                return self_color * intersect.object.unwrap().get_texture().alpha
                    + new_color * (1.0 - intersect.object.unwrap().get_texture().alpha);
            }
            self_color
        } else {
            let skybox_pos = self.skybox_position(ray);
            self.skybox.texture(skybox_pos.x, skybox_pos.y).as_vector()
        }
    }

    fn check_pixels_proximity(&self, samples: &Vec<Vector>) -> bool {
        let px: Vector = self.combine_pixel(samples);
        for sample in samples {
            if (((sample.x as i16 - px.x as i16)
                + (sample.y as i16 - px.y as i16)
                + (sample.z as i16 - px.y as i16)) as i64)
                .abs()
                > self.camera.super_sampling_precision as i64
            {
                return false;
            }
        }
        true
    }

    pub fn render_pixel(&self, x: i64, y: i64, config: &Config) -> Vector {
        if config.fast_mode != 0 {
            return self.get_color_from_ray_fast(
                self.camera.transform.pos,
                self.camera.get_pixel_vectors(x, y, 1)[0],
            );
        }

        let mut samples: Vec<Vector> = Vec::new();
        let mut camera_to_pixel_vector =
            self.camera
                .get_pixel_vectors(x, y, self.camera.super_sampling);
        for i in 0..camera_to_pixel_vector.len() {
            let mut recursion = Recursivity {
                general: self.camera.recursivity,
                transmission: 10,
            };
            samples.push(self.get_color_from_ray(
                self.camera.transform.pos,
                camera_to_pixel_vector[i],
                &mut recursion,
            ));

            if self.camera.super_sampling > 4 && i == 3 && self.check_pixels_proximity(&samples) {
                for _ in 4..self.camera.super_sampling {
                    camera_to_pixel_vector.push(self.camera.get_random_pixel_vector(x, y))
                }
            }
        }
        self.combine_pixel(&samples)
    }

    pub fn naive_thread_renderer(
        &self,
        pixel_states: Arc<Mutex<Vec<bool>>>,
        pixels: Arc<Mutex<Vec<u8>>>,
        progression: Arc<Mutex<u64>>,
        config: &Config,
    ) {
        let mut pixel_id: usize;
        let mut line_state_id: usize;

        for i in 0..self.camera.lens.height {
            line_state_id = i as usize;
            let mut locked_pixel_states = pixel_states.lock().unwrap(); // lock

            if locked_pixel_states[line_state_id] {
                continue;
            }
            locked_pixel_states[line_state_id] = true;
            drop(locked_pixel_states); // nécessaire pour laisser les autres threads bouger dès que possible

            let mut local_pixel_line: Vec<u8> = vec![0; (self.camera.lens.width * 3) as usize];
            for j in 0..self.camera.lens.width {
                pixel_id = (j * 3) as usize;
                let calculated_pixel = self.render_pixel(j, i, config);

                local_pixel_line[pixel_id] =
                    (self.camera.aces_curve(calculated_pixel.x).powf(1.0 / 2.2) * 255.0) as u8;
                local_pixel_line[pixel_id + 1] =
                    (self.camera.aces_curve(calculated_pixel.y).powf(1.0 / 2.2) * 255.0) as u8;
                local_pixel_line[pixel_id + 2] =
                    (self.camera.aces_curve(calculated_pixel.z).powf(1.0 / 2.2) * 255.0) as u8;
            }
            let mut locked_pixels = pixels.lock().unwrap(); // lock
            for k in 0..(self.camera.lens.width * 3) {
                pixel_id = (k + (i * self.camera.lens.width * 3)) as usize;
                locked_pixels[pixel_id] = local_pixel_line[k as usize];
            }

            if self.camera.progression {
                let mut locked_progression = progression.lock().unwrap();
                *locked_progression += 1;
            }
        }
    }

    pub fn print_progression(
        &self,
        progression: Arc<Mutex<u64>>,
        buf_step: u64,
        buf_size: u64,
        config: &Config,
    ) {
        let mut last_progression = 0_u64;

        while last_progression != self.camera.lens.height as u64 {
            if config.fast_mode == 0 {
                thread::sleep(time::Duration::from_millis(250));
            } else {
                thread::sleep(time::Duration::from_millis(50));
            }
            let locked_progression = progression.lock().unwrap();
            print!("rendered [");
            for _i in 0..(((*locked_progression + (self.camera.lens.height as u64 * buf_step))
                * 100)
                / (self.camera.lens.height as u64 * buf_size))
            {
                print!("#");
            }
            for _i in 0..(100
                - (((*locked_progression + (self.camera.lens.height as u64 * buf_step)) * 100)
                    / (self.camera.lens.height as u64 * buf_size)))
            {
                print!(" ");
            }
            println!(
                "] {:?}/{:?}\r",
                (*locked_progression + (self.camera.lens.height as u64 * buf_step)),
                (self.camera.lens.height as u64 * buf_size)
            );
            last_progression = *locked_progression;
        }
    }

    pub fn merge_image(&self, last_image: &mut Vec<u8>, new_image: &[u8], image_nbr: u64) {
        for i in 0..last_image.len() {
            last_image[i] = (((last_image[i] as u64 * (image_nbr - 1)) + new_image[i] as u64)
                / image_nbr) as u8;
        }
    }

    pub fn pull_new_image(&self, config: &Config) -> Vec<u8> {
        let pixels: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![
            0;
            (self.camera.lens.height * self.camera.lens.width * 3)
                as usize
        ]));
        let pixels_state: Arc<Mutex<Vec<bool>>> =
            Arc::new(Mutex::new(vec![false; self.camera.lens.height as usize]));
        let progression: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

        thread::scope(|scope| {
            for _ in 0..self.camera.threads {
                let clone_pixels = Arc::clone(&pixels);
                let clone_pixels_state = Arc::clone(&pixels_state);
                let clone_progression = Arc::clone(&progression);
                scope.spawn(move || {
                    self.naive_thread_renderer(
                        clone_pixels_state,
                        clone_pixels,
                        clone_progression,
                        config,
                    );
                });
            }

            if self.camera.progression {
                self.print_progression(progression, 0, 1, config);
            }
        });
        let final_pixels = pixels.lock().unwrap().to_vec();
        final_pixels
    }

    pub fn render(&self, config: &Config) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let buf_size = if config.fast_mode != 0 {
            1
        } else {
            self.camera.image_buffer_size
        };

        for n in 0..buf_size {
            let pixels: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![
                0;
                (self.camera.lens.height * self.camera.lens.width * 3)
                    as usize
            ]));
            let pixels_state: Arc<Mutex<Vec<bool>>> =
                Arc::new(Mutex::new(vec![false; self.camera.lens.height as usize]));
            let progression: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

            thread::scope(|scope| {
                for _ in 0..self.camera.threads {
                    let clone_pixels = Arc::clone(&pixels);
                    let clone_pixels_state = Arc::clone(&pixels_state);
                    let clone_progression = Arc::clone(&progression);
                    scope.spawn(move || {
                        self.naive_thread_renderer(
                            clone_pixels_state,
                            clone_pixels,
                            clone_progression,
                            config,
                        );
                    });
                }

                if self.camera.progression {
                    self.print_progression(progression, 0, buf_size, config);
                }
            });

            let final_pixels = pixels.lock().unwrap().to_vec();

            if result.len() != final_pixels.len() {
                for pixel in final_pixels.iter() {
                    result.push(*pixel);
                }
            } else {
                for i in 0..result.len() {
                    result[i] = (((result[i] as u64 * (n - 1)) + final_pixels[i] as u64) / n) as u8;
                }
            }
        }
        result
    }

    pub fn get_renderer_from_file(config: &Config) -> Option<Renderer> {
        let mut _result: Option<Renderer> = None;
        let parser = Parser {};
        if parser.get_json(&config.config_file).is_some() {
            _result = Some(parser.get_renderer_from_json(
                &parser.get_json(&config.config_file).unwrap(),
                config.height,
                config.width,
            ));
            return _result;
        }
        None
    }
}
