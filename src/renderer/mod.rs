//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// renderer
//

pub mod primitives;

use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use crate::vectors;
use vectors::Vector;
use crate::renderer::primitives::{Object, Sphere, Light, Intersection};

#[derive(Debug, Clone)]
pub struct Transform {
    pos: vectors::Vector,
    rotation : vectors::Vector,
    scale : vectors::Vector,
}

impl Transform {
    pub fn new(
        x_pos: f64,
        y_pos: f64,
        z_pos: f64,
        x_rot: f64,
        y_rot: f64,
        z_rot: f64,
        x_sca: f64,
        y_sca: f64,
        z_sca: f64,
    ) -> Self {
        Transform {
            pos: Vector {
                x: x_pos,
                y: y_pos,
                z: z_pos,
            },
            rotation: Vector {
                x: x_rot,
                y: y_rot,
                z: z_rot,
            },
            scale: Vector {
                x: z_sca,
                y: z_sca,
                z: z_sca,
            },
        }
    }
}

#[derive(Debug)]
pub struct Renderer {
    camera: Camera,
    objects: Vec<Sphere>,
    lights: Vec<Light>,
}

#[derive(Debug)]
struct Lens {
    height: i64,
    width: i64,
    distance: f64,
    vector_to_first_pixel: Vector,
}

#[derive(Debug)]
pub struct Camera {
    transform : Transform,
    lens : Lens,
    fov : i16,
    smooth_shadow: bool,
    smooth_shadow_step: i16,
    diffuse: f64,
    ambient: f64,
    specular: f64,
}

impl Camera {
    fn new() -> Self {
        let mut result = Camera {
            transform: Transform::new(0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0, 0.0),
            fov: 80,
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.6,
            smooth_shadow_step: 1,
            smooth_shadow: true,
            lens: Lens {
                width: 1920,
                height: 1080,
                distance: 0.0,
                vector_to_first_pixel: Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        };
        result.calculate_lens_distance();
        let vector_director = Vector {x:0.0, y:result.lens.distance, z:0.0};
        result.lens.vector_to_first_pixel = Vector {x:result.transform.pos.x, y:result.transform.pos.y, z:result.transform.pos.z};
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + Vector {x:0.0, y:0.0, z:1.0} * (result.lens.height as f64 / 2.0);
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + vector_director;
        result.lens.vector_to_first_pixel = result.lens.vector_to_first_pixel + Vector {x:-1.0, y:0.0, z:0.0} * (result.lens.width as f64 / 2.0);
        result
    }

    fn get_pixel_vector(&self, x: i64, y: i64) -> Vector {
        let mut pixel_vector = self.lens.vector_to_first_pixel.clone();

        pixel_vector = pixel_vector + Vector {x:1.0, y:0.0, z:0.0} * x as f64;
        pixel_vector = pixel_vector + Vector {x:0.0, y:0.0, z:-1.0} * y as f64;
        pixel_vector.rotate(self.transform.rotation.x, self.transform.rotation.y, self.transform.rotation.z);
        pixel_vector.normalize()
    }

    fn calculate_lens_distance(&mut self) {
        self.lens.distance = (self.lens.width as f64 / 2.0) / (self.fov as f64 / 2.0).to_radians().tan();
    }

    pub fn calculate_tone_mapping(val: f64) -> f64{
        let a = 2.51;
        let b = 0.03;
        let c = 2.43;
        let d = 0.59;
        let e = 0.14;
        ((val * (a * val + b))/(val * (c * val + d) + e)).clamp(0.0, 1.0)
    }
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            camera: Camera::new(),
            objects: vec![
                Sphere {
                    origin: Vector {x:0.0, y:3.0, z:0.0},
                    radius: 1.0,
                    ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                }, Sphere {
                    origin: Vector {x:2.0, y:6.0, z:-2.0},
                    radius: 1.0,
                    ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                }, Sphere {
                    origin: Vector {x:7.0, y:20.0, z:-7.0},
                    radius: 2.0,
                    ambient: 0.3,
                    diffuse: 0.5,
                    specular: 0.4,
                    shininess: 4.0,
                    color: Vector {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                }
            ],
            lights: vec![ Light {
                origin: Vector {x:-4.0, y:-5.0, z:3.0},
                intensity: 80.0,
                color: Vector {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                radius: 0.1,
            }]
        }
    }

    fn calculate_light(&self, light: &Light, intersect: Intersection, camera_to_pixel: Vector, object: Sphere) -> Vector {
        let normal_vector = intersect.normal.normalize();
        let mut light_reached: i16 = 0;
        let mut light_vector = (light.origin - intersect.intersection_point).normalize();
        let mut light_uncovered = 1.0;

        if self.camera.smooth_shadow == false {
            for object_current in self.objects.iter() {
                if *object_current == object { continue; }
                let intersect = object_current.intersection(light_vector, intersect.intersection_point);
                if intersect != None { return Vector {x: 0.0, y: 0.0, z:0.0} }
            };
        } else {
            for _ in 0..self.camera.smooth_shadow_step {
                let light_vector = (light.origin + Vector::get_random_point_in_sphere(light.radius) - intersect.intersection_point).normalize();
                let mut intersected = true;
                for object_current in self.objects.iter() {
                    if *object_current == object { continue; }
                    let intersect = object_current.intersection(light_vector, intersect.intersection_point);
                    if intersect != None { intersected = false };
                };
                if intersected == true { light_reached += 1; }
            }
            light_vector = (light.origin - intersect.intersection_point).normalize();
            light_uncovered = light_reached as f64 / self.camera.smooth_shadow_step as f64;
        }
        let diffuse = light_vector.dot_product(normal_vector).max(0.0) * self.camera.diffuse * object.diffuse;
        let reflected = light_vector.reflect(normal_vector).normalize();
        let view = (camera_to_pixel * -1.0).normalize();
        let specular = self.camera.specular * object.specular * reflected.dot_product(view).max(0.0).powf(object.shininess);
        let distance = intersect.intersection_point.distance(light.origin);
        let light_falloff = (light.intensity / distance.powi(2)).max(0.0);
        object.color * light.color * diffuse * light_falloff * light_uncovered + light.color * specular * light_falloff * light_uncovered
    }

    fn found_nearest_intersection(&self, camera_to_pixel: Vector) -> Option<Intersection> {
        let mut found_intersection: Option<Intersection> = None;
        let mut smallest_distance: f64 = f64::INFINITY;

        for object in self.objects.iter() {
            let intersect = object.intersection(camera_to_pixel, self.camera.transform.pos);

            if intersect != None {

                let distance_found = (intersect.unwrap().intersection_point - self.camera.transform.pos).len();
                if distance_found < smallest_distance {
                    smallest_distance = distance_found;
                    found_intersection = intersect;
                }
            }
        }
        found_intersection
    }

    pub fn render_pixel(&self, x:i64, y:i64) -> [u8; 3] {
        let mut pixel:[u8; 3] = [0; 3];

        let camera_to_pixel = self.camera.get_pixel_vector(x, y);
        let intersect = self.found_nearest_intersection(camera_to_pixel);
        if intersect != None {
            let mut color = intersect.unwrap().object.color * self.camera.ambient * intersect.unwrap().object.ambient;
            for light in self.lights.iter() {
                color = color + self.calculate_light(light, intersect.unwrap(), camera_to_pixel, intersect.unwrap().object);
            }
            pixel[0] = ((color.x).clamp(0.0, 1.0) * 255.0) as u8;
            pixel[1] = ((color.x).clamp(0.0, 1.0) * 255.0) as u8;
            pixel[2] = ((color.x).clamp(0.0, 1.0) * 255.0) as u8;
        } else {
            let color_a = Vector {x: 0.0, y: 212.0, z: 255.0} * (1.0/255.0);
            let color_b = Vector {x: 2.0, y: 0.0, z: 36.0} * (1.0/255.0);
            let percent = y as f64 / self.camera.lens.height as f64;
            let result = color_a + (color_b - color_a) * percent as f64;
            pixel[0] = (result.x * 255.0 as f64) as u8;
            pixel[1] = (result.y * 255.0 as f64) as u8;
            pixel[2] = (result.z * 255.0 as f64) as u8;
        }
        pixel
    }

    pub fn naive_thread_renderer(&self, pixel_states:Arc<Mutex<Vec<bool>>>, pixels:Arc<Mutex<Vec<u8>>>, id:u8) {
        //println!("thread {id:?} started");
        let mut calculated_pixel: [u8; 3]; // variable où sera stockée un pixel tout juste calculé
        let mut pixel_id: usize;
        let mut line_state_id: usize;

        for i in 0..(self.camera.lens.height) {
            let test_size = self.camera.lens.height * self.camera.lens.width * 3;

            line_state_id = i as usize;
            let mut locked_pixel_states = pixel_states.lock().unwrap(); // lock

            if locked_pixel_states[line_state_id] == true {
                drop (locked_pixel_states); // optionnel vu qu'on reset la scope du for ?
                continue;
            }
            //println!("thread {id:?} on pixel {line_state_id:?}");
            locked_pixel_states[line_state_id] = true;
            drop (locked_pixel_states); // nécéssaire pour laisser les autres threads bouger dès que possible

            let mut local_pixel_line: Vec<u8> = vec![0; (self.camera.lens.width * 3) as usize];
            for j in 0..self.camera.lens.width {
                pixel_id = (j * 3) as usize;
                calculated_pixel = self.render_pixel(j, i);

                local_pixel_line[pixel_id + 0] = calculated_pixel[0];
                local_pixel_line[pixel_id + 1] = calculated_pixel[1];
                local_pixel_line[pixel_id + 2] = calculated_pixel[2];
            }
            let mut locked_pixels = pixels.lock().unwrap(); // lock
            for k in 0..(self.camera.lens.width * 3) {
                pixel_id = ((k + (i * self.camera.lens.width * 3))) as usize;
                locked_pixels[pixel_id as usize] = local_pixel_line[k as usize];
            }
            drop(locked_pixels); // optionnel vu qu'on reset la scope du for ?
        }
    }

// o..o..o..o..
// o..o..o..o..
// o..o..o..o..
// o..o..o..o..


    pub fn render(&self) -> Vec<u8> {
        let pixels:Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; (self.camera.lens.height * self.camera.lens.width * 3) as usize]));
        let pixels_state:Arc<Mutex<Vec<bool>>> = Arc::new(Mutex::new(vec![false; self.camera.lens.height as usize]));

        thread::scope(|scope| {
            for i in 0..8 {
                let clone_pixels = Arc::clone(&pixels);
                let clone_pixels_state = Arc::clone(&pixels_state);
                scope.spawn(move || {
                    self.naive_thread_renderer(clone_pixels_state, clone_pixels, i);
                });
            }
        });
        let final_pixels = pixels.lock().unwrap().to_vec();
        final_pixels
    }

    pub fn render_old(&self) -> Vec<u8> {
        let mut pixels:Vec<u8> = Vec::new();

        for i in 0..self.camera.lens.height {
            for j in 0..self.camera.lens.width {
                let camera_to_pixel = self.camera.get_pixel_vector(j, i);
                let intersect = self.found_nearest_intersection(camera_to_pixel);
                if intersect != None {
                    let mut color = intersect.unwrap().object.color * self.camera.ambient * intersect.unwrap().object.ambient;
                    for light in self.lights.iter() {
                        color = color + self.calculate_light(light, intersect.unwrap(), camera_to_pixel, intersect.unwrap().object);
                    }
                    pixels.extend(&[
                        ((color.x).clamp(0.0, 1.0) * 255.0) as u8,
                        ((color.y).clamp(0.0, 1.0) * 255.0) as u8,
                        ((color.z).clamp(0.0, 1.0) * 255.0) as u8
                    ]);
                } else {
                    let color_a = Vector {x: 0.0, y: 212.0, z: 255.0} * (1.0/255.0);
                    let color_b = Vector {x: 2.0, y: 0.0, z: 36.0} * (1.0/255.0);
                    let percent = i as f64 / self.camera.lens.height as f64;
                    let result = color_a + (color_b - color_a) * percent as f64;
                    pixels.extend(&[
                        (result.x * 255.0 as f64) as u8,
                        (result.y * 255.0 as f64) as u8,
                        (result.z * 255.0 as f64) as u8
                    ]);
                }
            }
        }
        pixels
    }
}
