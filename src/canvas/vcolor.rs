use std::ops;

use num_traits::Float;

use crate::zequality::ZEq;

#[derive(Debug, Clone, Copy)]
pub struct VColor<T>
where
    T: Float,
{
    pub r: T,
    pub g: T,
    pub b: T,
}
impl<T: Float> VColor<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
    pub fn black() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
    pub fn red() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }
    pub fn green() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }
    pub fn blue() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }
    pub fn to_rgb_str(&self) -> String {
        let convert = |f: T| -> u8 { (f.round().to_u8().unwrap() * 255).clamp(0, 255) };
        return format!(
            "{} {} {}",
            convert(self.r),
            convert(self.g),
            convert(self.b)
        );
    }
    pub fn clamp(&self, lower_bound: T, upper_bound: T) -> VColor<T> {
        VColor::new(
            self.r.min(upper_bound).max(lower_bound),
            self.g.min(upper_bound).max(lower_bound),
            self.b.min(upper_bound).max(lower_bound),
        )
    }
}
/*
    VColor operators implementation
*/
impl<T> ops::Add<Self> for VColor<T>
where
    T: Float,
{
    type Output = Self;

    fn add(self, other: VColor<T>) -> Self::Output {
        VColor::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl<T> ops::Sub<Self> for VColor<T>
where
    T: Float,
{
    type Output = Self;

    fn sub(self, other: VColor<T>) -> Self::Output {
        VColor::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}
impl<T> ops::Mul<T> for VColor<T>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, multiplier: T) -> Self::Output {
        VColor::new(
            self.r * multiplier,
            self.g * multiplier,
            self.b * multiplier,
        )
    }
}
impl<T> ops::Mul<VColor<T>> for VColor<T>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, other: VColor<T>) -> Self::Output {
        VColor::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
// Perhaps implement own assert_zeq! with custom zequal trait and macro?
impl<T> PartialEq for VColor<T>
where
    T: Float,
{
    fn eq(&self, other: &VColor<T>) -> bool {
        self.r.zeq(other.r) && self.g.zeq(other.g) && self.b.zeq(other.b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_are_rgb_tuples() {
        let c = VColor::new(-0.5, 0.4, 1.7);

        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }
    #[test]
    fn adding_colors() {
        let c1 = VColor::new(0.9, 0.6, 0.75);
        let c2 = VColor::new(0.7, 0.1, 0.25);

        let result = c1 + c2;
        let expected_result = VColor::new(1.6, 0.7, 1.0);

        assert_eq!(result, expected_result);
    }
    #[test]
    fn subtracting_colors() {
        let c1 = VColor::new(0.9, 0.6, 0.75);
        let c2 = VColor::new(0.7, 0.1, 0.25);

        let result = c1 - c2;
        let expected_result = VColor::new(0.2, 0.5, 0.5);

        assert_eq!(result, expected_result);
    }
    #[test]
    fn multiplying_color_by_a_scalar() {
        let c = VColor::new(0.2, 0.3, 0.4);
        let scalar = 2.0;

        let result = c * scalar;
        let expected_result = VColor::new(0.4, 0.6, 0.8);

        assert_eq!(result, expected_result);
    }
    #[test]
    fn multiplying_colors() {
        let c1 = VColor::new(1.0, 0.2, 0.4);
        let c2 = VColor::new(0.9, 1.0, 0.1);

        let result = c1 * c2;
        let expected_result = VColor::new(0.9, 0.2, 0.04);

        assert_eq!(result, expected_result);
    }
}
