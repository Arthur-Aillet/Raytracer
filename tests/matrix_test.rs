//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// matrix tests
//

use raytracer::matrix::Matrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_new() {
        let matrix = Matrix::new(2, 3);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 3);
        assert_eq!(matrix.data, vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]]);
    }

    #[test]
    fn test_matrix_mul_scalar() {
        let matrix = Matrix::new(2, 2);
        let matrix_scaled = matrix * 2.0;
        assert_eq!(matrix_scaled.data, vec![vec![0.0, 0.0], vec![0.0, 0.0]]);
    }

    #[test]
    fn test_matrix_mul_matrix() {
        let matrix1 = Matrix {
            rows: 2,
            cols: 3,
            data: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
        };
        let matrix2 = Matrix {
            rows: 3,
            cols: 2,
            data: vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]],
        };
        let matrix_result = matrix1 * matrix2;
        assert_eq!(
            matrix_result.data,
            vec![vec![58.0, 64.0], vec![139.0, 154.0]]
        );
    }

    #[test]
    fn test_matrix_euler_rotation() {
        let mut matrix = Matrix::euler_rotation(0.0, 90.0, 0.0);

        for i in 0..9 {
            matrix.data[i / 3][i % 3] =
                (matrix.data[i / 3][i % 3] * 10000000.0).round() / 10000000.0;
        }
        assert_eq!(
            matrix.data,
            vec![
                vec![0.0, 0.0, 1.0],
                vec![0.0, 1.0, 0.0],
                vec![-1.0, 0.0, 0.0]
            ]
        );
    }
}
