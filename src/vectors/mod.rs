//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// vector
//

use std::f64::consts::PI;
use crate::matrix;
use matrix::Matrix;
use std::ops::{Add, Mul, Sub, Div};
use rand::Rng;

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

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
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

    pub fn get_random_point_in_sphere (radius: f64) -> Vector {
        let mut rng = rand::thread_rng();
        let theta = rng.gen_range(0.0..PI * 2.0);
        let v: f64 = rng.gen_range(0.0..1.0);
        let phi = ((2.0 * v) - 1.0).acos();
        let r = (rng.gen_range(0.0..1.0) as f64).powf(1.0/3.0);
        Vector {
            x: r * phi.sin() * theta.cos() * radius,
            y: r * phi.sin() * theta.sin() * radius,
            z: r * phi.cos() * radius,
        }
    }

    pub fn dot_product(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
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
        let len = self.len();
        Vector {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn distance(&self, other: Vector) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }

    pub fn len(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

pub fn number_of_solution(a: f64, b: f64, c: f64) -> i8 {
    let delta: f64 = (b.powi(2)) - (4 as f64 * a * c);

    return if delta < 0 as f64 {
        0
    } else if delta == 0 as f64 {
        1
    } else {
        2
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
