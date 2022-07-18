use std::ops;

use crate::zequality::ZEq;

#[derive(Debug, Clone, Copy)]
pub struct VColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl VColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    pub fn black() -> Self {
        Self::new( 0.0, 0.0, 0.0 )
    }
    pub fn red() -> Self {
        Self::new( 1.0, 0.0, 0.0 )
    }
    pub fn green() -> Self {
        Self::new( 0.0, 1.0, 0.0 )
    }
    pub fn blue() -> Self {
        Self::new( 0.0, 0.0, 1.0 )
    }
    pub fn to_rgb_str(&self)->String{
        let convert = |f:f64|->u8{((f*255.0).round()).clamp(0.0,255.0) as u8};
        return format!("{} {} {}", convert(self.r),convert(self.g),convert(self.b))
    }
}
/*
    VColor operators implementation
*/
impl ops::Add<Self> for VColor {
    type Output = Self;

    fn add(self, other: VColor) -> Self::Output {
        VColor::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}
impl ops::Sub<Self> for VColor {
    type Output = Self;

    fn sub(self, other: VColor) -> Self::Output {
        VColor::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}
impl ops::Mul<f64> for VColor {
    type Output = Self;

    fn mul(self, multiplier: f64) -> Self::Output {
        VColor::new(
            self.r * multiplier,
            self.g * multiplier,
            self.b * multiplier,
        )
    }
}
impl ops::Mul<VColor> for VColor {
    type Output = Self;

    fn mul(self, other: VColor) -> Self::Output {
        VColor::new(
            self.r * other.r,
            self.g * other.g,
            self.b * other.b,
        )
    }
}
// Perhaps implement own assert_zeq! with custom zequal trait and macro?
impl PartialEq for VColor {
    fn eq(&self, other: &Self) -> bool {
        self.r.zeq(&other.r)
            && self.g.zeq(&other.g)
            && self.b.zeq(&other.b)
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

        let result = c*scalar;
        let expected_result = VColor::new(0.4, 0.6, 0.8);

        assert_eq!(result, expected_result);
    }
    #[test]
    fn multiplying_colors() {
        let c1 = VColor::new(1.0, 0.2, 0.4);
        let c2 = VColor::new(0.9, 1.0, 0.1);

        let result = c1*c2;
        let expected_result = VColor::new(0.9, 0.2, 0.04);

        assert_eq!(result, expected_result);
    }

}