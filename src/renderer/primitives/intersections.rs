//
// EPITECH PROJECT, 2023
// Rustracer
// File description:
// intersections
//

mod primitives {
    impl Object for Sphere {
        fn intersection(&self, ray: VectorF) -> bool {
            return true;
        }
    }

    impl Object for Plan {
        fn intersection(&self, ray: VectorF) -> bool {
            return true;
        }
    }
}
