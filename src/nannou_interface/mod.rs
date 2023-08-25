// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// nannou interface module

mod layout;
mod components;

use nannou::image;
use nannou::prelude::*;
use nannou::App;
use nannou::Frame;
use std::env;

use crate::config;
use crate::config::Config;
use crate::ppm_interface::PPMInterface;
use crate::renderer::types::Transform;
use crate::renderer::Renderer;

use layout::Layout;

// Model struct for nannou_interface

pub struct Model {
    pub window: WindowId,
    pub draw: Draw,
    pub layout: Layout,
    pub config: Config,
    pub base_fast_mode: i64,
    pub last_image: Vec<u8>,
    pub image_nbr: u64,
    pub img_buf: String,
    pub camera_transform: Transform,
    pub fov: i64,
    pub exit: bool,
}

// model function for nannou_interface

fn model(app: &App) -> Model {
    let image_buffer = ".raytracer/imageBuffer.ppm";
    let args: Vec<String> = env::args().collect();
    let mut config = config::Config::from_args(&args);
    let renderer = Renderer::get_renderer_from_file(&config);
    if renderer.is_none() {
        std::process::exit(84);
    }
    let layout = Layout::new(config.clone());
    let window = app
        .new_window()
        .title("Rustracer")
        .size(
            config.width as u32
                + if config.layout {
                    layout.rect.w() as u32
                } else {
                    0
                },
            config.height as u32,
        )
        .view(view)
        .event(event)
        .build()
        .expect("Failed to build the window");
    let last_image = vec![0; (config.height * config.width * 3) as usize];

    config.height /= if config.fast_mode == 0 {
        1
    } else {
        config.fast_mode
    };
    config.width /= if config.fast_mode == 0 {
        1
    } else {
        config.fast_mode
    };
    Model {
        window,
        draw: app.draw(),
        layout,
        config: config.clone(),
        base_fast_mode: if config.fast_mode == 0 {
            1
        } else {
            config.fast_mode
        },
        last_image,
        image_nbr: 0,
        img_buf: image_buffer.to_string(),
        camera_transform: Transform::default(),
        fov: renderer.unwrap().camera.fov,
        exit: false,
    }
}

pub fn fancy_to_fast(model: &mut Model) {
    model.config.fast_mode = model.base_fast_mode;
    model.image_nbr = 0;
    model.config.width /= model.base_fast_mode;
    model.config.height /= model.base_fast_mode;
    model.last_image = vec![0; (model.config.height * model.config.width * 3) as usize];
}

pub fn fast_to_fancy(model: &mut Model) {
    model.config.fast_mode = 0;
    model.image_nbr = 0;
    model.config.width *= model.base_fast_mode;
    model.config.height *= model.base_fast_mode;
    model.last_image = vec![0; (model.config.height * model.config.width * 3) as usize];
}

// Update function for nannou_interface

fn merge_camera_transform(renderer: &mut Renderer, model: &Model) {
    renderer.camera.fov = model.fov;
    renderer.camera.calculate_lens_distance();
    renderer.camera.calculate_lens_size();

    renderer.camera.transform.pos.x = model.camera_transform.pos.x;
    renderer.camera.transform.pos.y = model.camera_transform.pos.y;
    renderer.camera.transform.pos.z = model.camera_transform.pos.z;
    renderer.camera.transform.rotation.x = model.camera_transform.rotation.x;
    renderer.camera.transform.rotation.y = model.camera_transform.rotation.y;
    renderer.camera.transform.rotation.z = model.camera_transform.rotation.z;
    renderer.camera.transform.scale = model.camera_transform.scale;
}

fn merge_interactions_layout(app: &App, model: &mut Model) {
    if model
        .layout
        .get_buttons_interactions(app, "fast".to_string())
        && model.config.fast_mode == 0
    {
        fancy_to_fast(model);
    }
    if model
        .layout
        .get_buttons_interactions(app, "fancy".to_string())
        && model.config.fast_mode > 0
    {
        fast_to_fancy(model);
    }
    if model
        .layout
        .get_buttons_interactions(app, "exit".to_string())
    {
        std::process::exit(0);
    }
    if model.layout.get_sliders_interactions("fov".to_string()) != -1 {
        model.fov = model.layout.get_sliders_interactions("fov".to_string());
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let renderer = Renderer::get_renderer_from_file(&model.config);

    if let Some(mut render) = renderer {
        merge_camera_transform(&mut render, model);
        model.image_nbr += 1;
        if model.config.fast_mode == 0 {
            let new_image = render.pull_new_image(&model.config);
            render.merge_image(&mut model.last_image, &new_image, model.image_nbr);
        } else {
            model.last_image = render.pull_new_image(&model.config);
        }
        if model.config.layout {
            model.layout.display(_app, &model.draw, &render);
        }
    } else {
        println!("Error: Renderer not found");
    }
}

// Event function for nannou_interface

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    // Gérer les événements de la fenêtre comme la souris, le clavier, le redimensionnement, etc. ici.
    if let KeyPressed(key) = event {
        if key == Key::G {
            if model.config.fast_mode >= 1 {
                fast_to_fancy(model);
            } else {
                fancy_to_fast(model);
            }
        }
        if key == Key::Escape {
            std::process::exit(0);
        }
        if key == Key::Space {
            model.camera_transform.pos.z += 1.0;
        }
        if key == Key::LControl {
            model.camera_transform.pos.z -= 1.0;
        }
        if key == Key::Z {
            model.camera_transform.pos.y += 1.0;
        }
        if key == Key::S {
            model.camera_transform.pos.y -= 1.0;
        }
        if key == Key::D {
            model.camera_transform.pos.x += 1.0;
        }
        if key == Key::Q {
            model.camera_transform.pos.x -= 1.0;
        }
        if key == Key::Right {
            model.camera_transform.rotation.z -= 2.0;
        }
        if key == Key::Left {
            model.camera_transform.rotation.z += 2.0;
        }
        if key == Key::Down {
            model.camera_transform.rotation.x -= 2.0;
        }
        if key == Key::Up {
            model.camera_transform.rotation.x += 2.0;
        }
        if key == Key::A {
            model.camera_transform.rotation.y -= 2.0;
        }
        if key == Key::E {
            model.camera_transform.rotation.y += 2.0;
        }
        if key == Key::P {
            PPMInterface::new(&model.config.save_file).write(
                model.config.width,
                model.config.height,
                model.last_image.clone(),
            );
        }
    }
    merge_interactions_layout(_app, model);
}

pub fn draw_canvas(draw: &Draw, pixels: &[u8], model: &Model, app: &App) {
    let img_path = &model.img_buf;
    PPMInterface::new(img_path).write(model.config.width, model.config.height, pixels.to_vec());

    if image::open(img_path).is_ok() {
        let texture = wgpu::Texture::from_path(app, img_path).unwrap();
        let mut window_rect = app.window_rect();
        if model.config.layout {
            window_rect.x.end -= model.layout.rect.w();
        }
        draw.texture(&texture)
            .xy(window_rect.xy())
            .wh(window_rect.wh());
    } else {
        eprintln!("Failed to open image: {}", img_path);
    }
}

// Main view function for nannou_interface

fn view(app: &App, model: &Model, frame: Frame) {
    let color = nannou::color::rgb_u32(0x302B34);

    model.draw.background().color(color);
    draw_canvas(&model.draw, &model.last_image, model, app);
    model.draw.to_frame(app, &frame).unwrap();
}

pub fn run_nannou_interface() {
    nannou::app(model).update(update).run()
}
