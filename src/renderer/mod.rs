//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer common structures
//

mod camera;
mod primitives;
mod lights;
mod parsing;
mod renderer_common;
use serde::{Serialize};

use rand::Rng;
use crate::renderer::primitives::{Object, Intersection};
use crate::renderer::lights::Light;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};
use camera::{Camera};
use lights::Lights;
use parsing::Parser;
use crate::config::Config;
use crate::vectors::Vector;

#[derive(Serialize)]
pub struct Renderer {
    pub camera: Camera,
    pub primitives: Vec<Box<dyn Object + Send + Sync>>,
    pub lights: Lights,
}

impl Renderer {
    pub fn new(height: i64, width: i64) -> Renderer {
        Renderer {
            camera: Camera::default(height, width),
            primitives: Vec::new(),
            lights: Lights {
                lights: Vec::new(),
                ambient: Vec::new(),
            },
        }
    }

    fn found_nearest_intersection_fast(&self, origin: Vector, ray: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

        for object in self.primitives.iter() {
            let intersect = object.intersection(ray, origin);

            if intersect.is_some() {
                let inters = intersect.unwrap();
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

            if intersect.is_some() {
                let inters = intersect.unwrap();
                let distance_found = (inters.intersection_point - origin).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }
        for light in self.lights.lights.iter() {
            if light.get_visible() == false {
                continue;
            }
            let intersect = light.intersection(ray, origin);

            if intersect.is_some() {
                let inters = intersect.unwrap();
                let distance_found = (inters.intersection_point - origin).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = Some(inters);
                }
            }
        }
        found_intersection
    }

    fn get_ambient<'a>(&self, object :&'a dyn Object) -> Vector {
        let mut self_color = Vector{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        for ambient in self.lights.ambient.iter() {
            self_color = self_color + object.get_texture().color.as_vector() * object.get_texture().ambient * ambient.color.as_vector() * ambient.strength * self.camera.ambient;
        }
        self_color
    }

    fn combine_pixel(&self, samples: &Vec::<Vector>) -> Vector {
        let mut result: Vector = samples[0];

        for i in 1..samples.len() {
            result = ((result * (i - 1) as f64) + (samples[i])) / i as f64;
        }
        result
    }

    fn get_color_from_ray_fast(&self, origin: Vector, ray: Vector) -> Vector {
        let maybe_intersect = self.found_nearest_intersection_fast(origin, ray);

        if let Some(intersect) = maybe_intersect {
            let normal_vector = intersect.normal.normalize();
            let light_vector = (self.camera.transform.pos - intersect.intersection_point).normalize();

            let ambient = intersect.object.unwrap().get_texture().color.as_vector() * intersect.object.unwrap().get_texture().ambient * self.camera.ambient;

            let diffuse = light_vector.dot_product(normal_vector).max(0.0) * self.camera.diffuse * intersect.object.unwrap().get_texture().diffuse;

            ambient + intersect.object.unwrap().get_texture().color.as_vector() * diffuse
        } else {
            Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }

    fn get_color_from_ray(&self, origin: Vector, ray: Vector, recursivity: i64) -> Vector {
        if recursivity == 0 {
           return Vector {
               x: 0.0,
               y: 0.0,
               z: 0.0,
           }
        }
        let maybe_intersect = self.found_nearest_intersection(origin, ray);

        if let Some(intersect) = maybe_intersect {
            // case of direct intersection with light object
            if let Some(light_touched) = intersect.light {
                return light_touched.get_color().as_vector();
            }

            let mut self_color = self.get_ambient(intersect.object.unwrap());

            // calculation of lighting
            for light in self.lights.lights.iter() {
                self_color = self_color + light.calculate_light(&intersect, ray, self.camera, &self.primitives);
            }

            let surface_point = intersect.intersection_point + intersect.normal * self.camera.shadow_bias;

            self_color = self_color * (1.0 - intersect.object.unwrap().get_texture().metalness);
            if recursivity == 1 {
                return self_color;
            }
            let samples_nbr = (1.0 + self.camera.reflection_samples as f64 * intersect.object.unwrap().get_texture().roughness).powf(intersect.object.unwrap().get_texture().sampling_ponderation);
            for _ in 0..samples_nbr as i32 {
                let mut rng = rand::thread_rng();
                // random vector used for the roughness
                let random_a: f64 = rng.gen_range(0.0..6.28);
                let random_b: f64 = rng.gen_range(0.0..6.28);
                let random_vect = Vector {
                    x: random_a.cos() * random_b.cos(),
                    y: random_a.sin() * random_b.cos(),
                    z: random_b.sin()
                };
                let mut reflection_ray = (ray.normalize() - intersect.normal.normalize() * 2.0 * intersect.normal.dot_product(ray.normalize())).normalize();
                if intersect.object.unwrap().get_texture().roughness != 0.0 {
                    reflection_ray.lerp(&random_vect, intersect.object.unwrap().get_texture().roughness);
                }
                let metalness = intersect.object.unwrap().get_texture().metalness;

                let new_color = self.get_color_from_ray(surface_point, reflection_ray, recursivity - 1);

                self_color =
                    self_color
                    + (((new_color * (1.0 - metalness) * intersect.object.unwrap().get_texture().specular))
                    + (new_color * intersect.object.unwrap().get_texture().color.as_vector() * metalness))
                    * (1.0/samples_nbr as f64);
            }
            self_color
        } else {
            Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }

    fn check_pixels_proximity(&self, samples: &Vec<Vector>) -> bool {
        let px: Vector = self.combine_pixel(samples);
        for i in 0..samples.len() {
            if (((
                    samples[i].x as i16 - px.x as i16) +
                    (samples[i].y as i16 - px.y as i16) +
                    (samples[i].z as i16 - px.y as i16)) as i64).abs() > self.camera.super_sampling_precision as i64 {
                return false
            }
        }
        true
    }

    pub fn render_pixel(&self, x:i64, y:i64, config: &Config) -> Vector {
        if config.fast_mode != 0 {
            return self.get_color_from_ray_fast(self.camera.transform.pos, self.camera.get_pixel_vectors(x, y, 1)[0]);
        }

        let mut samples : Vec<Vector> = Vec::new();
        let mut camera_to_pixel_vector = self.camera.get_pixel_vectors(x, y, self.camera.super_sampling);
        for i in 0..camera_to_pixel_vector.len() {
            samples.push(self.get_color_from_ray(self.camera.transform.pos, camera_to_pixel_vector[i], self.camera.recursivity));

            if self.camera.super_sampling > 4 && i == 3 && self.check_pixels_proximity(&samples) {
                for _ in 4..self.camera.super_sampling {camera_to_pixel_vector.push(self.camera.get_random_pixel_vector(x, y))}
            }
        }
        self.combine_pixel(&samples)
    }

    pub fn naive_thread_renderer(&self, pixel_states:Arc<Mutex<Vec<bool>>>, pixels:Arc<Mutex<Vec<u8>>>, progression:Arc<Mutex<u64>>, config: &Config) {
        //println!("thread {id:?} started");
        let mut pixel_id: usize;
        let mut line_state_id: usize;

        for i in 0..self.camera.lens.height {
            line_state_id = i as usize;
            let mut locked_pixel_states = pixel_states.lock().unwrap(); // lock

            if locked_pixel_states[line_state_id] == true {
                continue;
            }
            locked_pixel_states[line_state_id] = true;
            drop (locked_pixel_states); // nécessaire pour laisser les autres threads bouger dès que possible

            let mut local_pixel_line: Vec<u8> = vec![0; (self.camera.lens.width * 3) as usize];
            for j in 0..self.camera.lens.width {
                pixel_id = (j * 3) as usize;
                let calculated_pixel = self.render_pixel(j, i, &config);

                local_pixel_line[pixel_id + 0] = (self.camera.aces_curve(calculated_pixel.x).powf(1.0/2.2) * 255.0) as u8;
                local_pixel_line[pixel_id + 1] = (self.camera.aces_curve(calculated_pixel.y).powf(1.0/2.2) * 255.0) as u8;
                local_pixel_line[pixel_id + 2] = (self.camera.aces_curve(calculated_pixel.z).powf(1.0/2.2) * 255.0) as u8;
            }
            let mut locked_pixels = pixels.lock().unwrap(); // lock
            for k in 0..(self.camera.lens.width * 3) {
                pixel_id = (k + (i * self.camera.lens.width * 3)) as usize;
                locked_pixels[pixel_id as usize] = local_pixel_line[k as usize];
            }

            if self.camera.progression {
                let mut locked_progression = progression.lock().unwrap();
                *locked_progression += 1;
            }
        }
    }

    pub fn print_progression(&self, progression:Arc<Mutex<u64>>, buf_step: u64, buf_size: u64) {
        let mut last_progression:u64 = 0;

        while last_progression as u64 != self.camera.lens.height as u64 {
            thread::sleep(time::Duration::from_millis(1000));
            let locked_progression = progression.lock().unwrap();
            print!("rendered [");
            for _i in 0..(((*locked_progression + (self.camera.lens.height as u64 * buf_step)) * 100) / (self.camera.lens.height as u64 * buf_size)) {
                print!("#");
            }
            for _i in 0..(100 - (((*locked_progression  + (self.camera.lens.height as u64 * buf_step)) * 100) / (self.camera.lens.height as u64 * buf_size))) {
                print!(" ");
            }
            println!("] {:?}/{:?}\r", (*locked_progression  + (self.camera.lens.height as u64 * buf_step)), (self.camera.lens.height as u64 * buf_size));
            last_progression = *locked_progression;
        }
    }

    pub fn render(&self, config: &Config) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let buf_size = if config.fast_mode != 0 { 1 } else { self.camera.image_buffer_size };
        for n in 0..buf_size {
            let pixels:Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; (self.camera.lens.height * self.camera.lens.width * 3) as usize]));
            let pixels_state:Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(vec![false; self.camera.lens.height as usize]));
            let progression:Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
            thread::scope(|scope| {
                for _ in 0..self.camera.threads {
                    let clone_pixels = Arc::clone(&pixels);
                    let clone_pixels_state = Arc::clone(&pixels_state);
                    let clone_progression = Arc::clone(&progression);
                    scope.spawn(move || {
                        self.naive_thread_renderer(clone_pixels_state, clone_pixels, clone_progression, &config);
                    });
                }

                if self.camera.progression == true {
                    self.print_progression(progression, n, buf_size);
                }
            });
            let final_pixels = pixels.lock().unwrap().to_vec();
            if result.len() != final_pixels.len() {
                for i in 0..final_pixels.len() {
                    result.push(final_pixels[i]);
                }
            } else{
                for i in 0..result.len() {
                    result[i] = (((result[i] as u64 * (n - 1)) + final_pixels[i] as u64) / n) as u8;
                }
            }
        }
        result
    }

    pub fn get_renderer_from_file(config: &Config) -> Option<Renderer> {
        let mut _result: Option<Renderer> = None;
        let parser = Parser{};
        if parser.get_json(&config.config_file).is_some() {
            _result = Some(parser.get_renderer_from_json(&parser.get_json(&config.config_file).unwrap(), config.height, config.width));
            return _result
        }
        None
    }
}
