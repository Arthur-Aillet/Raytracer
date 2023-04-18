//
// EPITECH PROJECT, 2023
// Rustracer Major
// File description:
// matrix
//

use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, scale: f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] * scale;
            }
        }
        result
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        self.clone().multiply(&other)
    }
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![vec![0.0; cols]; rows];
        Matrix { rows, cols, data }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);

        let mut result = Matrix::new(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    pub fn euler_rotation(phi: f64, theta: f64, psi: f64) -> Matrix {
        let cos_phi = phi.to_radians().cos();
        let sin_phi = phi.to_radians().sin();
        let cos_theta = theta.to_radians().cos();
        let sin_theta = theta.to_radians().sin();
        let cos_psi = psi.to_radians().cos();
        let sin_psi = psi.to_radians().sin();

        let mut result = Matrix::new(3, 3);

        result.data[0][0] = cos_psi * cos_theta;
        result.data[0][1] = -sin_psi * cos_phi + cos_psi * sin_theta * sin_phi;
        result.data[0][2] = sin_psi * sin_phi + cos_psi * sin_theta * cos_phi;
        result.data[1][0] = sin_psi * cos_theta;
        result.data[1][1] = cos_psi * cos_phi + sin_psi * sin_theta * sin_phi;
        result.data[1][2] = -cos_psi * sin_phi + sin_psi * sin_theta * cos_phi;
        result.data[2][0] = -sin_theta;
        result.data[2][1] = cos_theta * sin_phi;
        result.data[2][2] = cos_theta * cos_phi;

        result
    }
}
