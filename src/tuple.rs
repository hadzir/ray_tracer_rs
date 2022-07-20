use std::ops;

use crate::F;

use crate::zequality::ZEq;

#[derive(Debug, Copy, Clone)]
pub struct VTuple {
    pub x: F,
    pub y: F,
    pub z: F,
    pub w: F,
}
/*
    VTuple type implementation
*/
impl VTuple {
    pub fn new(x: F, y: F, z: F, w: F) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    pub fn point(x: F, y: F, z: F) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }
    pub fn vector(x: F, y: F, z: F) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }
    pub fn is_point(&self) -> bool {
        return self.w != 0.0;
    }
    pub fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }

    pub fn magnitude(&self) -> F {
        return (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt();
    }

    pub fn normalize(&self) -> Self {
        return *self / self.magnitude();
    }

    pub fn dot(&self, &other: &Self) -> F {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    pub fn cross(&self, &other: &Self) -> Self {
        if !(self.is_vector() && other.is_vector()) {
            panic!("Cross product can only be calculated for two errors")
        }

        return VTuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        );
    }
}

impl ops::Add<Self> for VTuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        VTuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}
impl ops::Sub<Self> for VTuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        VTuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl ops::Neg for VTuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        VTuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}
impl ops::Mul<F> for VTuple {
    type Output = Self;

    fn mul(self, multiplier: F) -> Self::Output {
        VTuple::new(
            self.x * multiplier,
            self.y * multiplier,
            self.z * multiplier,
            self.w * multiplier,
        )
    }
}
impl ops::Div<F> for VTuple {
    type Output = Self;

    fn div(self, divisor: F) -> Self::Output {
        VTuple::new(
            self.x / divisor,
            self.y / divisor,
            self.z / divisor,
            self.w / divisor,
        )
    }
}
// Perhaps implement own assert_zeq! with custom zequal trait and macro?
impl ZEq<Self> for VTuple
where
    F: ZEq<F>,
{
    fn zeq(&self, other: Self) -> bool {
        self.x.zeq(other.x) && self.y.zeq(other.y) && self.z.zeq(other.z) && self.w.zeq(other.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_fills_values_correctly() {
        let point = VTuple::point(1.0, 2.0, 3.0);
        assert_eq!(point.x, 1.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
        assert_eq!(point.w, 1.0);
    }
    #[test]
    fn vector_fills_values_correctly() {
        let vector = VTuple::vector(1.0, 2.0, 3.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 0.0);
    }
    #[test]
    fn point_type_signal_is_correct() {
        let point = VTuple::point(1.0, 2.0, 3.0);
        assert!(point.is_point())
    }
    #[test]
    fn vector_type_signal_is_correct() {
        let vector = VTuple::vector(1.0, 2.0, 3.0);
        assert!(vector.is_vector())
    }
    #[test]
    fn tuples_can_be_added() {
        let tuple1 = VTuple::new(1.0, 1.0, 1.0, 1.0);
        let tuple2 = VTuple::new(2.0, 2.0, 2.0, 2.0);
        let expected_tuple = VTuple::new(3.0, 3.0, 3.0, 3.0);
        assert_zeq!(tuple1 + tuple2, expected_tuple)
    }
    #[test]
    fn tuples_can_be_subtracted() {
        let tuple1 = VTuple::point(2.0, 2.0, 2.0);
        let tuple2 = VTuple::point(1.0, 1.0, 1.0);
        let expected_result = VTuple::vector(1.0, 1.0, 1.0);
        let result = tuple1 - tuple2;

        assert!(result.is_vector());
        assert_zeq!(result, expected_result)
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = VTuple::point(3.0, 2.0, 1.0);
        let v = VTuple::vector(5.0, 6.0, 7.0);

        let expected_result = VTuple::point(-2.0, -4.0, -6.0);
        let result = p - v;

        assert!(result.is_point());
        assert_zeq!(result, expected_result)
    }
    #[test]
    fn subtracting_two_vector() {
        let v1 = VTuple::vector(3.0, 2.0, 1.0);
        let v2 = VTuple::vector(5.0, 6.0, 7.0);

        let expected_result = VTuple::vector(-2.0, -4.0, -6.0);
        let result = v1 - v2;

        assert!(result.is_vector());
        assert_zeq!(result, expected_result)
    }
    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let v1 = VTuple::vector(0.0, 0.0, 0.0);
        let v2 = VTuple::vector(1.0, 2.0, 3.0);

        let expected_result = VTuple::vector(-1.0, -2.0, -3.0);
        let result = v1 - v2;

        assert!(result.is_vector());
        assert_zeq!(result, expected_result)
    }
    #[test]
    fn negating_a_tuple() {
        let v1 = VTuple::new(1.0, 2.0, 3.0, 4.0);

        let expected_result = VTuple::new(-1.0, -2.0, -3.0, -4.0);
        let result = -v1;

        assert_zeq!(result, expected_result)
    }
    #[test]
    fn scalar_multiplication() {
        let multiplier = 2.0;
        let v1 = VTuple::new(1.0, 2.0, 3.0, 4.0);

        let expected_result = VTuple::new(2.0, 4.0, 6.0, 8.0);
        let result = v1 * multiplier;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let multiplier = 1.0 / 2.0;
        let v1 = VTuple::new(1.0, 2.0, 3.0, 4.0);

        let expected_result = VTuple::new(0.5, 1.0, 1.5, 2.0);
        let result = v1 * multiplier;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let dividor = 2.0;
        let v1 = VTuple::new(1.0, 2.0, 3.0, 4.0);

        let expected_result = VTuple::new(0.5, 1.0, 1.5, 2.0);
        let result = v1 / dividor;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn dividing_a_tuple_by_a_fraction() {
        let dividor = 0.5;
        let v1 = VTuple::new(1.0, 2.0, 3.0, 4.0);

        let expected_result = VTuple::new(2.0, 4.0, 6.0, 8.0);
        let result = v1 / dividor;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn compute_magnitude_of_vector_1_0_0() {
        let v = VTuple::vector(1.0, 0.0, 0.0);

        let result = v.magnitude();
        let expected_result = 1.0;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn compute_magnitude_of_vector_3_4_0() {
        let v = VTuple::vector(3.0, 4.0, 0.0);

        let result = v.magnitude();
        let expected_result = 5.0;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn compute_magnitude_of_vector_0_3_4() {
        let v = VTuple::vector(0.0, 3.0, 4.0);

        let result = v.magnitude();
        let expected_result = 5.0;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn compute_magnitude_of_vector_1_2_3() {
        let v = VTuple::vector(1.0, 2.0, 3.0);

        let result = v.magnitude();
        let expected_result = (14.0 as f64).sqrt();

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn compute_magnitude_of_vector_n1_n2_n3() {
        let v = VTuple::vector(-1.0, -2.0, -3.0);

        let result = v.magnitude();
        let expected_result = (14.0 as f64).sqrt();

        assert_zeq!(result, expected_result);
    }

    #[test]
    fn normalize_vector_4_0_0() {
        let v = VTuple::vector(4.0, 0.0, 0.0);

        let result = v.normalize();
        let expected_result = VTuple::vector(1.0, 0.0, 0.0);

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn normalize_vector_1_2_3() {
        let v = VTuple::vector(1.0, 2.0, 3.0);

        let result = v.normalize();
        let expected_result = VTuple::vector(0.26726, 0.53452, 0.80178);

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn magnitude_of_normalized_vector_is_1() {
        let v = VTuple::vector(1.0, 2.0, 3.0);

        let result = v.normalize().magnitude();
        let expected_result = 1.0;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = VTuple::vector(1.0, 2.0, 3.0);
        let v2 = VTuple::vector(2.0, 3.0, 4.0);

        let result = v1.dot(&v2);
        let expected_result = 20.0;

        assert_zeq!(result, expected_result);
    }
    #[test]
    fn cross_product_of_two_vectors1() {
        let v1 = VTuple::vector(1.0, 2.0, 3.0);
        let v2 = VTuple::vector(2.0, 3.0, 4.0);

        let result = v1.cross(&v2);
        let expected_result = VTuple::vector(-1.0, 2.0, -1.0);

        assert!(result.is_vector());
        assert_zeq!(result, expected_result);
    }
    #[test]
    fn cross_product_of_two_vectors2() {
        let v1 = VTuple::vector(1.0, 2.0, 3.0);
        let v2 = VTuple::vector(2.0, 3.0, 4.0);

        let result = v2.cross(&v1);
        let expected_result = VTuple::vector(1.0, -2.0, 1.0);

        assert!(result.is_vector());
        assert_zeq!(result, expected_result);
    }
}
