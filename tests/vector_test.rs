//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// vector tests
//

use raytracer::vector;
use vector::resolve_quadratic_equation;
use vector::Vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let fst = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let snd = Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        let result = fst + snd;

        assert_eq!(
            result,
            Vector {
                x: 5.0,
                y: 7.0,
                z: 9.0,
            }
        );
    }

    #[test]
    fn test_rotate() {
        let mut vec = Vector {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        vec.rotate(0.0, 0.0, 90.0);
        assert_eq!(vec.x as f32, -2.);
        assert_eq!(vec.y as f32, 1.);
        assert_eq!(vec.z as f32, 3.);
        /* float error far in the decimals, could be neglected
        // { x: -2.0, y: 1.0000000000000002, z: 3.0 }
        assert_eq!(
            vec,
            Vector {
                x: -2.0,
                y: 1.0,
                z: 3.0,
            }
        );
        */
    }

    #[test]
    fn test_dot_product() {
        let p1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let p2 = Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        assert_eq!(p1.dot_product(p2), 32.0);

        let p1 = Vector {
            x: -1.0,
            y: 0.0,
            z: 2.0,
        };
        let p2 = Vector {
            x: 3.0,
            y: 4.0,
            z: -5.0,
        };
        assert_eq!(p1.dot_product(p2), -13.0);

        let p1 = Vector {
            x: 1.5,
            y: 2.5,
            z: -3.5,
        };
        let p2 = Vector {
            x: 0.5,
            y: -0.5,
            z: 1.5,
        };
        assert_eq!(p1.dot_product(p2), -5.75);
    }

    #[test]
    fn test_resolve_quadratic_equation() {
        // Test case 1: No roots
        let a = 1.0;
        let b = 0.0;
        let c = 1.0;
        let result = resolve_quadratic_equation(a, b, c);
        assert!(result.is_empty());

        // Test case 2: One root
        let a = 1.0;
        let b = -2.0;
        let c = 1.0;
        let result = resolve_quadratic_equation(a, b, c);
        assert_eq!(result, vec![1.0]);

        // Test case 3: Two roots
        let a = 1.0;
        let b = -3.0;
        let c = 2.0;
        let result = resolve_quadratic_equation(a, b, c);
        assert_eq!(result, vec![2.0, 1.0]);
    }

    #[test]
    fn test_reflect_vector() {
        let vec = Vector {
            x: 2.0,
            y: 4.0,
            z: 4.0,
        };

        let refer = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        assert_eq!(
            vec.reflect(refer),
            Vector {
                x: -2.0,
                y: -4.0,
                z: 4.0,
            }
        );
    }
}
