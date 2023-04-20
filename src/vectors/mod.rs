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
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Vector {
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

    pub fn dot_product(&self, other: Vector) -> f64 {
        let dx = self.x * other.x;
        let dy = self.y * other.y;
        let dz = self.z * other.z;
        dx + dy + dz
    }

    pub fn reflect(&self, reference: Vector) -> Self {
        let reflected = reference * 2.0 * (self.dot_product(reference));
        Vector {
            x: reflected.x - self.x,
            y: reflected.y - self.y,
            z: reflected.z - self.z,
        }
    }

    pub fn normalize(&self) -> Self {
        Vector {
            x: self.x / self.len(),
            y: self.y / self.len(),
            z: self.z / self.len(),
        }
    }

    pub fn len(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub origin : Vector,
    pub end: Vector,
}

impl Add<Segment> for Segment {
    type Output = Segment;
    fn add(self, other: Segment) -> Segment {
        Segment {
            origin: Vector {
                x: self.origin.x,
                y: self.origin.y,
                z: self.origin.z,
            },
            end: Vector {
                x: self.end.x + other.end.x - other.origin.x,
                y: self.end.y + other.end.y - other.origin.y,
                z: self.end.z + other.end.z - other.origin.z,
            },
        }
    }
}

impl Mul<f64> for Segment {
    type Output = Segment;
    fn mul(self, other: f64) -> Segment {
        Segment {
            origin: Vector {
                x: self.origin.x,
                y: self.origin.y,
                z: self.origin.z,
            },
            end: Vector {
                x: self.origin.x + (self.end.x - self.origin.x) * other,
                y: self.origin.y + (self.end.y - self.origin.y) * other,
                z: self.origin.z + (self.end.z - self.origin.z) * other,
            },
        }
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        let vec1: Segment = self.to_origin();
        let vec2: Segment = other.to_origin();
        vec1.end == vec2.end
    }
}

impl Segment {
    pub fn new(x:f64, y:f64, z:f64) -> Segment {
        Segment { origin: Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, end: Vector {
            x,
            y,
            z,
        }
        }
    }

    pub fn rotate(&mut self, x: f64, y: f64, z: f64) {
        let mut direction_matrix = Matrix::new(3, 1);
        direction_matrix.data[0][0] = self.end.x;
        direction_matrix.data[1][0] = self.end.y;
        direction_matrix.data[2][0] = self.end.z;

        let rotation_matrix = Matrix::euler_rotation(x, y, z);
        let rotated_direction_matrix = rotation_matrix.multiply(&direction_matrix);
        self.end.x = rotated_direction_matrix.data[0][0];
        self.end.y = rotated_direction_matrix.data[1][0];
        self.end.z = rotated_direction_matrix.data[2][0];
    }

    pub fn add(&mut self, other: Segment) {
        self.origin = Vector {
            x: self.origin.x,
            y: self.origin.y,
            z: self.origin.z,
        };
        self.end = Vector {
            x: self.end.x + other.end.x - other.origin.x,
            y: self.end.y + other.end.y - other.origin.y,
            z: self.end.z + other.end.z - other.origin.z,
        }
    }

    pub fn to_origin(&self) -> Segment {
        Segment { origin: Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }, end: Vector {
            x: self.end.x - self.origin.x,
            y: self.end.y - self.origin.y,
            z: self.end.z - self.origin.z,
        }
        }
    }

    pub fn len(&self) -> f64 {
        let origin_v = self.to_origin();
        (origin_v.end.x.powi(2) + origin_v.end.y.powi(2) + origin_v.end.z.powi(2)).sqrt()
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
