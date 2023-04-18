//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// vector tests
//

use raytracer::vectors;
use vectors::Point;
use vectors::VectorF;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let vec1 = VectorF {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction: Point {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        };

        let vec2 = VectorF {
            origin: Point {
                x: 7.0,
                y: 8.0,
                z: 9.0,
            },
            direction: Point {
                x: 10.0,
                y: 11.0,
                z: 12.0,
            },
        };

        let result = vec1.clone() + vec2.clone();

        assert_eq!(
            result.origin,
            Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
        assert_eq!(
            result.direction,
            Point {
                x: 7.0,
                y: 8.0,
                z: 9.0,
            }
        );
    }

    #[test]
    fn test_rotate() {
        let mut vec = VectorF {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction: Point {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        };
        vec.rotate(0.0, 0.0, 90.0);
        assert_eq!(
            vec.direction,
            Point {
                x: -5.0,
                y: 4.0,
                z: 6.0,
            }
        );
    }

    #[test]
    fn test_add_in_place() {
        let vec1 = VectorF {
            origin: Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            direction: Point {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        };

        let vec2 = VectorF {
            origin: Point {
                x: 7.0,
                y: 8.0,
                z: 9.0,
            },
            direction: Point {
                x: 10.0,
                y: 11.0,
                z: 12.0,
            },
        };

        let mut vec3 = vec1.clone();
        vec3.add(vec2);

        assert_eq!(
            vec3.origin,
            Point {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            }
        );
        assert_eq!(
            vec3.direction,
            Point {
                x: 7.0,
                y: 8.0,
                z: 9.0,
            }
        );
    }
}
