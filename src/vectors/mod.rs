//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// vector
//

use crate::matrix;
use matrix::Matrix;
use std::ops::{Add, Mul, Sub};
use std::os::unix::raw::off_t;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Point {
    pub fn rotate(&mut self, x: f64, y: f64, z: f64) {
        let mut direction_matrix = Matrix::new(3, 1);
        direction_matrix.data[0][0] = self.x;
        direction_matrix.data[1][0] = self.y;
        direction_matrix.data[2][0] = self.z;

        let rotation_matrix = Matrix::euler_rotation(x, y, z);
        let rotated_direction_matrix = rotation_matrix.multiply(&direction_matrix);
        self.x = rotated_direction_matrix.data[0][0];
        self.y = rotated_direction_matrix.data[1][0];
        self.z = rotated_direction_matrix.data[2][0];
    }

    pub fn dot_product(&self, other: Point) -> f64 {
        let dx = self.x * other.x;
        let dy = self.y * other.y;
        let dz = self.z * other.z;
        dx + dy + dz
    }

    pub fn reflect(&mut self, reference: Point) {
        let reflected = reference * 2.0 * (self.dot_product(reference));
        self.x = reflected.x - self.x;
        self.y = reflected.y - self.y;
        self.z = reflected.z - self.z;
    }

    pub fn normalize(&mut self) -> Point {
        self.x = self.x/self.len();
        self.y = self.y/self.len();
        self.z = self.z/self.len();
        Point {x:self.x, y:self.y, z:self.z}
    }

    pub fn len(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VectorF {
    pub origin : Point,
    pub direction: Point,
}

impl Add<VectorF> for VectorF {
    type Output = VectorF;
    fn add(self, other: VectorF) -> VectorF {
        VectorF {
            origin: Point {
                x: self.origin.x,
                y: self.origin.y,
                z: self.origin.z,
            },
            direction: Point {
                x: self.direction.x + other.direction.x - other.origin.x,
                y: self.direction.y + other.direction.y - other.origin.y,
                z: self.direction.z + other.direction.z - other.origin.z,
            },
        }
    }
}

impl Mul<f64> for VectorF {
    type Output = VectorF;
    fn mul(self, other: f64) -> VectorF {
        VectorF {
            origin: Point {
                x: self.origin.x,
                y: self.origin.y,
                z: self.origin.z,
            },
            direction: Point {
                x: self.origin.x + (self.direction.x - self.origin.x) * other,
                y: self.origin.y + (self.direction.y - self.origin.y) * other,
                z: self.origin.z + (self.direction.z - self.origin.z) * other,
            },
        }
    }
}

impl PartialEq for VectorF {
    fn eq(&self, other: &Self) -> bool {
        let vec1: VectorF = self.to_origin();
        let vec2: VectorF = other.to_origin();
        vec1.direction == vec2.direction
    }
}

impl VectorF {
    pub fn new(x:f64, y:f64, z:f64) -> VectorF {
        VectorF { origin: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, direction: Point {
            x,
            y,
            z,
        }
        }
    }

    pub fn rotate(&mut self, x: f64, y: f64, z: f64) {
        let mut direction_matrix = Matrix::new(3, 1);
        direction_matrix.data[0][0] = self.direction.x;
        direction_matrix.data[1][0] = self.direction.y;
        direction_matrix.data[2][0] = self.direction.z;

        let rotation_matrix = Matrix::euler_rotation(x, y, z);
        let rotated_direction_matrix = rotation_matrix.multiply(&direction_matrix);
        self.direction.x = rotated_direction_matrix.data[0][0];
        self.direction.y = rotated_direction_matrix.data[1][0];
        self.direction.z = rotated_direction_matrix.data[2][0];
    }

    pub fn add(&mut self, other: VectorF) {
        self.origin = Point {
            x: self.origin.x,
            y: self.origin.y,
            z: self.origin.z,
        };
        self.direction = Point {
            x: self.direction.x + other.direction.x - other.origin.x,
            y: self.direction.y + other.direction.y - other.origin.y,
            z: self.direction.z + other.direction.z - other.origin.z,
        }
    }

    pub fn to_origin(&self) -> VectorF {
        VectorF { origin: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, direction: Point {
            x: self.direction.x - self.origin.x,
            y: self.direction.y - self.origin.y,
            z: self.direction.z - self.origin.z,
        }
        }
    }

    pub fn len(&self) -> f64 {
        let origin_v = self.to_origin();
        (origin_v.direction.x.powi(2) + origin_v.direction.y.powi(2) + origin_v.direction.z.powi(2)).sqrt()
    }
}

pub fn number_of_solution(a: f64, b: f64, c: f64) -> i8 {
    let delta: f64 = (b.powi(2)) - (4 as f64 * a * c);

    if delta < 0 as f64 {
        return 0;
    } else if delta == 0 as f64 {
        return 1;
    } else {
        return 2
    }
}

pub fn resolve_quadratic_equation(a: f64, b: f64, c: f64) -> Vec<f64> {
    let delta: f64 = (b.powi(2)) - (4 as f64 * a * c);
    let mut results: Vec<f64> = Vec::new();

    if delta == 0 as f64 {
        results.push(-b / (2 as f64 * a));
    } else if delta > 0 as f64 {
        results.push((-b + delta.sqrt()) / (2 as f64 * a));
        results.push((-b - delta.sqrt()) / (2 as f64 * a));
    }
    return results;
}
