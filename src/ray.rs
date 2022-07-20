use crate::{matrix::VMatrix, tuple::*, F};

#[derive(Debug, Copy, Clone, PartialEq)]

pub struct VRay {
    pub origin: VTuple,
    pub direction: VTuple,
}
impl VRay {
    pub fn new(origin: VTuple, direction: VTuple) -> Self {
        if !(origin.is_point() && direction.is_vector()) {
            panic!("Origin needs to be a point, and direction needs to be a vector")
        }
        Self { origin, direction }
    }
    pub fn position(&self, t: F) -> VTuple {
        self.origin + self.direction * t
    }
    pub fn transform(mut self, transformation_matrix: VMatrix<4>) {
        self.origin = transformation_matrix * self.origin;
        self.direction = transformation_matrix * self.direction;
    }
    pub fn transformed(&self, transformation_matrix: VMatrix<4>) -> Self {
        Self::new(
            transformation_matrix * self.origin,
            transformation_matrix * self.direction,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::*;

    #[test]
    fn creating_and_quering_a_ray() {
        let o = VTuple::point(1.0, 2.0, 3.0);
        let d = VTuple::vector(4.0, 5.0, 6.0);
        let r = VRay::new(o, d);

        assert_zeq!(r.origin, o);
        assert_zeq!(r.direction, d);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let o = VTuple::point(2.0, 3.0, 4.0);
        let d = VTuple::vector(1.0, 0.0, 0.0);
        let r = VRay::new(o, d);

        assert_zeq!(r.position(0.0), VTuple::point(2.0, 3.0, 4.0));
        assert_zeq!(r.position(1.0), VTuple::point(3.0, 3.0, 4.0));
        assert_zeq!(r.position(-1.0), VTuple::point(1.0, 3.0, 4.0));
        assert_zeq!(r.position(2.5), VTuple::point(4.5, 3.0, 4.0));
    }
    #[test]
    fn translating_a_ray() {
        let r = VRay::new(VTuple::point(1.0, 2.0, 3.0), VTuple::vector(0.0, 1.0, 0.0));
        let m = VMatrix::translation(3.0, 4.0, 5.0);
        let r2 = r.transformed(m);

        assert_zeq!(VTuple::point(4.0, 6.0, 8.0), r2.origin);
        assert_zeq!(VTuple::vector(0.0, 1.0, 0.0), r2.direction);
    }
    #[test]
    fn scaling_a_ray() {
        let r = VRay::new(VTuple::point(1.0, 2.0, 3.0), VTuple::vector(0.0, 1.0, 0.0));
        let m = VMatrix::scaling(2.0, 3.0, 4.0);
        let r2 = r.transformed(m);

        assert_zeq!(VTuple::point(2.0, 6.0, 12.0), r2.origin);
        assert_zeq!(VTuple::vector(0.0, 3.0, 0.0), r2.direction);
    }
}
