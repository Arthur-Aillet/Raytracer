//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// mod
//

mod vectors;

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Rotation {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Scale {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Transform {
    pos: Point,
    rotation: Rotation,
    scale: Scale,
}

pub struct Camera {
    transform: Transform,
    focal_length: f64,
    height: f64,
    width: f64,
    fov: i16,
}

pub struct Color {
    r: i8,
    g: i8,
    b: i8,
    a: i8,
}

pub struct Texture {
    texture_type: String,
    color: Color,
}

impl Texture {
    fn texture(&self, x: f64, y: f64) -> Color {
        if (texture_type == "color") {
            return (self.color);
        }
    }
}

pub struct Sphere {
    transform: Transform,
    texture: Texture,
    radius: f64,
}

impl Sphere {
    fn diameter(&self) -> f64 {
        radius * 2;
    }

    fn circumference(&self) -> f64 {
        radius * std::f64::consts::PI * 2
    }

    fn surface(&self) -> f64 {
        radius.powi(2) * std::f64::consts::PI * 4
    }

    fn volume(&self) -> f64 {
        ((std::f64::consts::PI * radius.powi(3)) * 4) / 3
    }
}

pub struct Plane {
    transform: Transform,
    texture: Texture,
    origin: Point,
    vector: Point,
}

impl Plane {
    fn surface(&self) -> f64 {
        height * width;
    }

    fn perimeter(&self) -> f64 {
        2 * (height + width);
    }
}

pub struct Cylinder {
    transform: Transform,
    texture: Texture,
    height: f64,
    radius: f64,
}

impl Cylinder {
    fn lateral_surface(&self) -> f64 {
        2 * std::f64::consts::PI * radius * height;
    }

    fn circle_surface(&self) -> f64 {
        std::f64::consts::PI * radius.powi(2);
    }

    fn total_surface(&self) -> f64 {
        2 * std::f64::consts::PI * radius * (height + radius);
    }

    fn volume(&self) -> f64 {
        std::f64::consts::PI * radius.powi(2) * height;
    }
}

pub struct Cone {
    transform: Transform,
    texture: Texture,
    radius: f64,
    height: f64,
}

impl Cone {
    fn slanted_height(&self) -> f64 {
        (radius.powi(2) + height.powi(2)).sqrt();
    }

    fn lateral_surface(&self) -> f64 {
        std::f64::consts::PI * radius * (radius.powi(2) + height.powi(2)).sqrt();
    }

    fn circle_surface(&self) -> f64 {
        std::f64::consts::PI * radius.powi(2);
    }

    fn total_surface(&self) -> f64 {
        std::f64::consts::PI * radius.powi(2) + std::f64::consts::PI * radius * (radius.powi(2) + height.powi(2)).sqrt();
    }

    fn volume(&self) -> f64 {
        (std::f64::consts::PI * radius.powi(2) * height) / 3;
    }
}

pub struct Primitives {
    spheres: Vec<Sphere>,
    planes: Vec<Plane>,
    cylinders: Vec<Cylinder>,
    cones: Vec<Cone>,
}

pub struct Directional {
    transform: Transform,
    color: Color,
    strength: f64,
}

pub struct Ambiant {
    color: Color,
    strength: f64,
}

pub struct Lights {
    directional: Vect<Directional>,
    ambiant: Vect<Ambiant>,
}

pub struct Renderer {
    camera: Camera,
    primitives: Primitives,
    lights: Lights,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
        }
    }

    pub fn render() -> Vec<u8> {
        let mut pixels = Vec::new();
    }
}
