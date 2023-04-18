//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// vector
//

use matrix::Matrix;
use crate::matrix;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
pub struct VectorF {
    pub origin : Point,
    pub direction: Point,
}

impl Add<VectorF> for VectorF {
    type Output = VectorF;
    fn add(self, other: VectorF) -> VectorF {
        VectorF {
            origin: Point {
                x: self.origin.x + other.origin.x,
                y: self.origin.y + other.origin.y,
                z: self.origin.z + other.origin.z,
            },
            direction: Point {
                x: self.direction.x + other.direction.x,
                y: self.direction.y + other.direction.y,
                z: self.direction.z + other.direction.z,
            },
        }
    }
}

impl VectorF {
    pub fn rotate(&mut self, x : f64, y : f64, z : f64) {
        let mut rotated = Matrix::new(3, 1);

        rotated.multiply(&Matrix::euler_rotation(x, y, z));
        self.direction.x = rotated.data[0][0];
        self.direction.y = rotated.data[0][1];
        self.direction.z = rotated.data[0][2];
    }
    pub fn add(mut self, other: VectorF) {
        self.origin = Point {
            x: self.origin.x + other.origin.x,
            y: self.origin.y + other.origin.y,
            z: self.origin.z + other.origin.z,
        };
        self.direction = Point {
            x: self.direction.x + other.direction.x,
            y: self.direction.y + other.direction.y,
            z: self.direction.z + other.direction.z,
        }
    }
    pub fn to_origin(self) -> Point {
        Point {
            x: self.direction.x - self.origin.x,
            y: self.direction.y - self.origin.y,
            z: self.direction.z - self.origin.z,
        }
    }
}

pub fn number_of_solution(a: f64, b: f64, c: f64) -> i8 {
    let delta: f64 = (b.powf(2 as f64)) - (4 as f64 * a * c);

    if delta < 0 as f64 {
        return 0;
    } else if delta == 0 as f64 {
        return 1;
    } else {
        return 2
    }
}

pub fn resolve_quadratic_equation(a: f64, b: f64, c: f64) -> Vec<f64> {
    let delta: f64 = (b.powf(2 as f64)) - (4 as f64 * a * c);
    let mut results: Vec<f64> = Vec::new();

    if delta == 0 as f64 {
        results.push(-b / (2 as f64 * a));
    } else if delta > 0 as f64 {
        results.push((-b + delta.sqrt()) / (2 as f64 * a));
        results.push((-b - delta.sqrt()) / (2 as f64 * a));
    }
    return results;
}
