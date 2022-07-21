use crate::{canvas::vcolor::VColor, tuple::VTuple};
#[derive(Clone, Copy)]
pub struct VPointLight {
    pub pos: VTuple,
    pub col: VColor,
}
impl VPointLight {
    pub fn new(pos: VTuple, col: VColor) -> Self {
        VPointLight { pos, col }
    }
}
impl Default for VPointLight {
    fn default() -> Self {
        Self::new(VTuple::point(0.0, 0.0, 0.0), VColor::white())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::zequality::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let col = VColor::new(1.0, 1.0, 1.0);
        let pos = VTuple::point(0.0, 0.0, 0.0);
        let l = VPointLight::new(pos, col);

        assert_zeq!(l.pos, pos);
        assert_zeq!(l.col, col);
    }
}
