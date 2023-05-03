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

use rand::Rng;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};
use crate::renderer::primitives::{Object, Intersection};
use std::fs;
use serde_json::Value;
use camera::{Camera};
use lights::Lights;
use parsing::Parser;
use crate::renderer::lights::Light;
use crate::vectors::Vector;


pub struct Renderer {
    pub camera: Camera,
    pub primitives: Vec<Box<dyn Object + Send + Sync>>,
    pub lights: Lights,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            camera: Camera::default(),
            primitives: Vec::new(),
            lights: Lights {
                lights: Vec::new(),
                ambient: Vec::new(),
            },
        }
    }

    fn light_is_intersected(&self, light_vector: Vector, intersect: &Intersection, light: &Box<dyn Light  + Send + Sync>, normal_vector: Vector) -> bool {
        for object_current in self.primitives.iter() {
            match object_current.intersection(light_vector, intersect.intersection_point + (normal_vector * self.camera.shadow_bias)) {
                None => { continue }
                Some(shadow_intersect) => {
                    if (shadow_intersect.intersection_point - intersect.intersection_point).len() < (light.get_transform().pos - intersect.intersection_point).len() {
                        return true
                    }
                }
            }
        }
        false
    }

    fn calculate_light(&self, light: &Box<dyn Light  + Send + Sync>, intersect: &Intersection, ray: Vector) -> Vector {
        let normal_vector = intersect.normal.normalize();
        let light_vector = (light.get_transform().pos - intersect.intersection_point).normalize();
        let mut light_uncovered = 1.0;

        if self.camera.smooth_shadow == false {
            if self.light_is_intersected(light_vector, intersect, light, normal_vector) {
                return Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }
        } else {
            let mut light_reached: i16 = 0;
            for _ in 0..self.camera.smooth_shadow_step {
                let inter_to_light = light.get_transform().pos + Vector::get_random_point_in_sphere(light.get_radius()) - intersect.intersection_point;
                if self.light_is_intersected(inter_to_light.normalize(), intersect, light, normal_vector) == false {
                    light_reached += 1;
                }
            }
            light_uncovered = light_reached as f64 / self.camera.smooth_shadow_step as f64;
        }
        let diffuse = light_vector.dot_product(normal_vector).max(0.0) * self.camera.diffuse * intersect.object.unwrap().get_texture().diffuse;

        let reflected = light_vector.reflect(normal_vector).normalize();
        let view = (ray * -1.0).normalize();
        let specular = self.camera.specular * intersect.object.unwrap().get_texture().specular * reflected.dot_product(view).max(0.0).powf(intersect.object.unwrap().get_texture().shininess);
        let distance = intersect.intersection_point.distance(light.get_transform().pos);
        let light_falloff = (light.get_strength() / distance.powi(light.get_falloff())).max(0.0);
        intersect.object.unwrap().get_texture().color.as_vector() * light.get_color().as_vector() * diffuse * light_falloff * light_uncovered + light.get_color().as_vector() * specular * light_falloff * light_uncovered
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
            if let Some(light_touched) = intersect.light {
                return light_touched.get_color().as_vector();
            }
            let mut self_color = self.get_ambient(intersect.object.unwrap());

            for light in self.lights.lights.iter() {
                self_color = self_color + self.calculate_light(light, &intersect, ray);
            }
            let surface_point = intersect.intersection_point + intersect.normal * self.camera.shadow_bias;

            self_color = self_color * (1.0 - intersect.object.unwrap().get_texture().metalness);
            let samples_nbr = 1.0 + self.camera.reflecion_samples as f64 * intersect.object.unwrap().get_texture().roughness;
            for _ in 0..samples_nbr as i32 {
                let mut rng = rand::thread_rng();
                let mut reflection_ray = (ray.normalize() - intersect.normal.normalize() * 2.0 * intersect.normal.dot_product(ray.normalize())).normalize();
                if intersect.object.unwrap().get_texture().roughness != 0.0 {
                    reflection_ray.rotate(rng.gen_range(0.0..90.0 * intersect.object.unwrap().get_texture().roughness), 0.0, rng.gen_range(0.0..360.0));
                }
                self_color = self_color + self.get_color_from_ray(surface_point, reflection_ray, recursivity - 1) * intersect.object.unwrap().get_texture().metalness * (1.0/samples_nbr as f64);
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

    pub fn naive_thread_renderer(&self, pixel_states:Arc<Mutex<Vec<bool>>>, pixels:Arc<Mutex<Vec<u8>>>, progression:Arc<Mutex<u64>>) {
        //println!("thread {id:?} started");
        let mut pixel_id: usize;
        let mut line_state_id: usize;

        for i in 0..(self.camera.lens.height) {
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
                let calculated_pixel = self.get_color_from_ray(self.camera.transform.pos, self.camera.get_pixel_vector(j, i), self.camera.recursivity);

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

    pub fn print_progression(&self, progression:Arc<Mutex<u64>>) {
        let mut last_progression:u64 = 0;

        while last_progression as i64 != self.camera.lens.height {
            thread::sleep(time::Duration::from_millis(1000));
            let locked_progression = progression.lock().unwrap();
            print!("rendered [");
            for i in 0..((*locked_progression * 100) / self.camera.lens.height as u64) {
                print!("#");
            }
            for i in 0..(100 - ((*locked_progression * 100) / self.camera.lens.height as u64)) {
                print!(" ");
            }
            println!("] {:?}/{:?}\r", *locked_progression, self.camera.lens.height);
            last_progression = *locked_progression;
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let pixels:Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; (self.camera.lens.height * self.camera.lens.width * 3) as usize]));
        let pixels_state:Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(vec![false; self.camera.lens.height as usize]));
        let progression:Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

        thread::scope(|scope| {
            for _ in 0..self.camera.threads {
                let clone_pixels = Arc::clone(&pixels);
                let clone_pixels_state = Arc::clone(&pixels_state);
                let clone_progression = Arc::clone(&progression);
                scope.spawn(move || {
                    self.naive_thread_renderer(clone_pixels_state, clone_pixels, clone_progression);
                });
            }

            if self.camera.progression == true {
                self.print_progression(progression);
            }

        });
        let final_pixels = pixels.lock().unwrap().to_vec();
        final_pixels
    }

    pub fn get_renderer_from_file(file: String) -> Renderer {
        let data = fs::read_to_string(file).expect("Unable to read file");
        let json: Value = serde_json::from_str(&data.to_string()).unwrap();
        let parser = Parser{};
        Renderer {
            camera: if json["camera"].is_object() {parser.get_camera_from_json(&json["camera"])} else {Camera::default()},
            primitives: if json["primitives"].is_object() {parser.get_objects_from_json(&json["primitives"])} else {Vec::new()},
            lights: if json["lights"].is_object() {parser.get_lights_from_json(&json["lights"])} else {Lights::default()},
        }
    }
}
