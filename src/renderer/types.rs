//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// common structures to the renderer
//

use crate::vector;
use nannou::image::io::Reader;

use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};
use vector::Vector;

#[derive(Debug, Clone, Copy)]
pub enum TexturesTypes {
    Color,
    Gradient,
    Perlin,
    Checkers,
    Image,
}

impl TexturesTypes {
    pub fn from_u64(src: u64) -> TexturesTypes {
        match src {
            0 => TexturesTypes::Color,
            1 => TexturesTypes::Gradient,
            2 => TexturesTypes::Perlin,
            3 => TexturesTypes::Checkers,
            4 => TexturesTypes::Image,
            _ => TexturesTypes::Color,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Transform {
    pub pos: Vector,
    pub rotation: Vector,
    pub scale: f64,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            pos: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: 1.0,
        }
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.rotation == other.rotation && self.scale == other.scale
    }
}

impl Add<Transform> for Transform {
    type Output = Transform;
    fn add(self, other: Transform) -> Transform {
        Transform {
            pos: self.pos + other.pos,
            rotation: self.rotation + other.rotation,
            scale: self.scale + other.scale,
        }
    }
}

impl Sub<Transform> for Transform {
    type Output = Transform;
    fn sub(self, other: Transform) -> Transform {
        Transform {
            pos: self.pos - other.pos,
            rotation: self.rotation - other.rotation,
            scale: self.scale - other.scale,
        }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, other: Transform) -> Transform {
        Transform {
            pos: self.pos * other.pos,
            rotation: self.rotation * other.rotation,
            scale: self.scale * other.scale,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 255.0,
            g: 255.0,
            b: 255.0,
        }
    }
}

impl Color {
    pub fn normal_map_default() -> Color {
        Color {
            r: 128.0,
            g: 128.0,
            b: 0.0,
        }
    }

    pub fn as_vector(self) -> Vector {
        Vector {
            x: self.r / 255.0,
            y: self.g / 255.0,
            z: self.b / 255.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    pub file: String,
    pub width: i64,
    pub height: i64,
}

impl Default for Image {
    fn default() -> Image {
        Image {
            file: "missing_image.ppm".to_string(),
            height: 0,
            width: 0,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Texture {
    pub texture_type: u64,
    pub color: Color,
    pub secondary_color: Color,
    pub image: Image,
    pub mod1: f64,
    pub mod2: f64,
    pub diffuse: f64,
    pub ambient: f64,
    pub specular: f64,
    pub metalness: f64,
    pub shininess: f64,
    pub roughness: f64,
    pub transmission: f64,
    pub ior: f64,
    pub sampling_ponderation: f64,
    pub alpha: f64,
}

impl Default for Texture {
    fn default() -> Texture {
        Texture {
            texture_type: TexturesTypes::Color as u64,
            color: Color::default(),
            secondary_color: Color::default(),
            image: Image::default(),
            mod1: 2.0,
            mod2: 2.0,
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.4,
            metalness: 0.1,
            shininess: 4.0,
            roughness: 0.25,
            transmission: 0.0,
            ior: 1.45,
            sampling_ponderation: 1.0,
            alpha: 1.0,
        }
    }
}

impl Texture {
    pub fn normal_map_default() -> Texture {
        Texture {
            texture_type: TexturesTypes::Color as u64,
            color: Color {
                r: 128.0,
                g: 128.0,
                b: 255.0,
            },
            secondary_color: Color {
                r: 128.0,
                g: 128.0,
                b: 0.0,
            },
            image: Image::default(),
            mod1: 2.0,
            mod2: 2.0,
            diffuse: 0.7,
            ambient: 0.1,
            specular: 0.4,
            metalness: 0.1,
            shininess: 4.0,
            roughness: 0.25,
            sampling_ponderation: 1.0,
            alpha: 255.0,
            transmission: 0.0,
            ior: 1.45,
        }
    }

    fn gradient_color(&self, _u: f64, v: f64) -> Color {
        Color {
            r: self.lerp(v, self.secondary_color.r, self.color.r),
            g: self.lerp(v, self.secondary_color.g, self.color.g),
            b: self.lerp(v, self.secondary_color.b, self.color.b),
        }
    }

    fn lerp(&self, t: f64, a: f64, b: f64) -> f64 {
        a + t * (b - a)
    }

    fn noise2(&self, x: i64, y: i64, hash: [i32; 256]) -> i32 {
        let tmp = hash[(y + self.mod1 as i64) as usize % 256];
        hash[(tmp + x as i32) as usize % 256]
    }

    fn lin_inter(&self, x: f64, y: f64, s: f64) -> f64 {
        x + s * (y - x)
    }

    fn smooth_inter(&self, x: f64, y: f64, s: f64) -> f64 {
        self.lin_inter(x, y, s * s * (3.0 - 2.0 * s))
    }

    fn noise(&self, x: f64, y: f64, hash: [i32; 256]) -> f64 {
        let x_int = x as i64;
        let y_int = y as i64;
        let x_frac = x - x_int as f64;
        let y_frac = y - y_int as f64;
        let s = self.noise2(x_int, y_int, hash) as f64;
        let t = self.noise2(x_int + 1, y_int, hash) as f64;
        let u = self.noise2(x_int, y_int + 1, hash) as f64;
        let v = self.noise2(x_int + 1, y_int + 1, hash) as f64;
        let low = self.smooth_inter(s, t, x_frac);
        let high = self.smooth_inter(u, v, x_frac);
        self.smooth_inter(low, high, y_frac)
    }

    fn perlin_noise(&self, x: f64, y: f64) -> Color {
        static HASH: [i32; 256] = [
            151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103,
            30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197,
            62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20,
            125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
            111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54,
            65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135,
            130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124,
            123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17,
            182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153,
            101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178,
            185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
            241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184,
            84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29,
            24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
        ];
        let mut xa = x * self.mod1;
        let mut ya = y * self.mod1;
        let mut amp = 1.0;
        let mut fin = 0.0;
        let mut div = 0.0;

        for _ in 0..self.mod2 as u64 {
            div += 256.0 * amp;
            fin += self.noise(xa, ya, HASH) * amp;
            amp /= 2.0;
            xa *= 2.0;
            ya *= 2.0;
        }
        Color {
            r: self.lerp(fin / div, self.color.r, self.secondary_color.r),
            g: self.lerp(fin / div, self.color.g, self.secondary_color.g),
            b: self.lerp(fin / div, self.color.b, self.secondary_color.b),
        }
    }

    fn checkers_color(&self, u: f64, v: f64) -> Color {
        if ((u * self.mod1) as i64 + (v * self.mod2) as i64) % 2 == 0 {
            self.color
        } else {
            self.secondary_color
        }
    }

    fn image_color(&self, u: f64, v: f64) -> Color {
        let img_x = (((1.0 - u) * self.mod1 % 1.0) * self.image.width as f64) as usize;
        let img_y = (((1.0 - v) * self.mod2 % 1.0) * self.image.height as f64) as usize;
        let mut reader = Reader::open(&self.image.file)
            .unwrap_or(
                Reader::open("assets/missing_texture.ppm").expect("missing missing texture texture\n"),
            )
            .decode()
            .expect("file invalid\n");
        if let Some(data) = reader
            .as_mut_rgb8()
            .expect("file invalid")
            .pixels()
            .nth(img_x + img_y * self.image.width as usize)
        {
            Color {
                r: data.0[0] as f64,
                g: data.0[1] as f64,
                b: data.0[2] as f64,
            }
        } else {
            Color::default()
        }
    }

    pub fn texture(&self, x: f64, y: f64) -> Color {
        match TexturesTypes::from_u64(self.texture_type) {
            TexturesTypes::Color => self.color,
            TexturesTypes::Gradient => self.gradient_color(x, y),
            TexturesTypes::Perlin => self.perlin_noise(x, y),
            TexturesTypes::Checkers => self.checkers_color(x, y),
            TexturesTypes::Image => self.image_color(x, y),
        }
    }
}
