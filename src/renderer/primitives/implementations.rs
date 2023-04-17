//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// implementations
//

mod primitives {
    trait Object {
        fn intersection(&self, ray: VectorF) -> bool;
    }

    pub struct Sphere {
        origin: Point,
        radius: f64,
    }

    pub struct Plan {
        origin: Point,
        endPoint: Point,
    }

    impl Sphere {
        fn set_origin(&mut self, origin: Point) {
            self.origin = origin;
        }
        fn set_radius(&mut self, radius: f64) {
            self.radius = radius;
        }
    }

    impl Plan {
        fn set_origin(&mut self, point: Point) {
            self.origin = point;
        }
        fn set_endPoint(&mut self, point: Point) {
            self.endPoint = point;
        }
    }
}
