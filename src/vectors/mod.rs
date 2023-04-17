//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// vector
//

pub struct Point {
    x: f64,
    y: f64,
}

pub struct VectorF {
    origin : Point,
    direction: Point,
}

pub fn number_of_solution(a: f64, b: f64, c: f64) -> u8 {
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
