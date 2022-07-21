use crate::light::VPointLight;
use crate::tuple::VTuple;
use crate::zequality::ZEq;
use crate::{canvas::vcolor::VColor, F};
pub trait Illuminated {
    fn lighting(&self, light: VPointLight, pos: VTuple, cam: VTuple, normal: VTuple) -> VColor;
}

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum VMaterial {
    VPhong(VPhong),
}
impl Illuminated for VMaterial {
    fn lighting(&self, light: VPointLight, pos: VTuple, cam: VTuple, normal: VTuple) -> VColor {
        match *self {
            VMaterial::VPhong(ref m) => m.lighting(light, pos, cam, normal),
        }
    }
}

impl Default for VMaterial {
    fn default() -> Self {
        VMaterial::from(VPhong::default())
    }
}
impl From<VPhong> for VMaterial {
    fn from(phong: VPhong) -> Self {
        VMaterial::VPhong(phong)
    }
}
impl ZEq<VMaterial> for VMaterial {
    fn zeq(&self, other: VMaterial) -> bool {
        match (self, other) {
            (VMaterial::VPhong(ref m), VMaterial::VPhong(other)) => m.zeq(other),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]

pub struct VPhong {
    pub col: VColor,
    pub amb: F,
    pub dif: F,
    pub spc: F,
    pub shi: F,
}
impl VPhong {
    pub fn new(color: VColor, ambient: F, diffuse: F, specular: F, shininess: F) -> Self {
        Self {
            col: color,
            amb: ambient,
            dif: diffuse,
            spc: specular,
            shi: shininess,
        }
    }
    pub fn default() -> VPhong {
        VPhong::new(VColor::white(), 0.1, 0.9, 0.9, 200.0)
    }
    pub fn with_color(mut self, col: VColor) -> VPhong {
        self.col = col;
        self
    }
    pub fn with_ambient(mut self, val: F) -> VPhong {
        self.amb = val;
        self
    }
    pub fn with_diffuse(mut self, val: F) -> VPhong {
        self.dif = val;
        self
    }
    pub fn with_specular(mut self, val: F) -> VPhong {
        self.spc = val;
        self
    }
    pub fn with_shininess(mut self, val: F) -> VPhong {
        self.shi = val;
        self
    }
}
impl ZEq<VPhong> for VPhong {
    fn zeq(&self, other: VPhong) -> bool {
        self.col.zeq(other.col)
            && self.amb.zeq(other.amb)
            && self.dif.zeq(other.dif)
            && self.spc.zeq(other.spc)
            && self.shi.zeq(other.shi)
    }
}
impl Illuminated for VPhong {
    fn lighting(&self, light: VPointLight, pos: VTuple, cam: VTuple, normal: VTuple) -> VColor {
        let light_amb: VColor;
        let light_dif: VColor;
        let light_spc: VColor;

        let eff_col = self.col * light.col;
        light_amb = eff_col * self.amb;

        let lightv = (light.pos - pos).normalize();
        let light_dot_normal = lightv.dot(&normal);
        if light_dot_normal < 0.0 {
            light_dif = VColor::black();
            light_spc = VColor::black();
        } else {
            light_dif = eff_col * self.dif * light_dot_normal;
            let reflectv = -lightv.reflected(normal);
            let reflect_dot_cam = reflectv.dot(&cam);
            if reflect_dot_cam <= 0.0 {
                light_spc = VColor::black();
            } else {
                let fac = reflect_dot_cam.powf(self.shi);
                light_spc = light.col * self.spc * fac;
            }
        }
        light_amb + light_dif + light_spc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{canvas::vcolor::VColor};

    #[test]
    fn default_phong_material() {
        let m = VPhong::default();

        assert_zeq!(m.col, VColor::white());
        assert_zeq!(m.amb, 0.1);
        assert_zeq!(m.dif, 0.9);
        assert_zeq!(m.spc, 0.9);
        assert_zeq!(m.shi, 200.0);
    }

    #[test]
    fn phong_material_can_be_constructed_with_builder() {
        let color = VColor::new(1.0, 1.0, 0.0);
        let ambient = 0.05;
        let diffuse = 0.7;
        let specular = 0.95;
        let shininess = 400.0;

        let m = VPhong::default()
            .with_color(color)
            .with_ambient(ambient)
            .with_diffuse(diffuse)
            .with_specular(specular)
            .with_shininess(shininess);

        assert_zeq!(m.col, color);
        assert_zeq!(m.amb, ambient);
        assert_zeq!(m.dif, diffuse);
        assert_zeq!(m.spc, specular);
        assert_zeq!(m.shi, shininess);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = VPhong::default();
        //let body = VBody::from(VSphere::default());
        let position = VTuple::point(0.0, 0.0, 0.0);

        let eyev = VTuple::vector(0.0, 0.0, -1.0);
        let normalv = VTuple::vector(0.0, 0.0, -1.0);
        let light = VPointLight::new(VTuple::point(0.0, 0.0, -10.0), VColor::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(light, position, eyev, normalv);

        let expected_result = VColor::new(1.9, 1.9, 1.9);

        assert_zeq!(actual_result, expected_result);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_by_45_degrees() {
        let m = VPhong::default();
        //let body = VBody::from(VSphere::default());
        let position = VTuple::point(0.0, 0.0, 0.0);

        let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
        let eyev = VTuple::vector(0.0, sqrt2_over_2, -sqrt2_over_2);
        let normalv = VTuple::vector(0.0, 0.0, -1.0);
        let light = VPointLight::new(VTuple::point(0.0, 0.0, -10.0), VColor::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(light, position, eyev, normalv);

        let expected_result = VColor::new(1.0, 1.0, 1.0);

        assert_zeq!(actual_result, expected_result);
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_by_45_degrees() {
        let m = VPhong::default();
        //let body = VBody::from(VSphere::default());
        let position = VTuple::point(0.0, 0.0, 0.0);

        let eyev = VTuple::vector(0.0, 0.0, -1.0);
        let normalv = VTuple::vector(0.0, 0.0, -1.0);
        let light = VPointLight::new(VTuple::point(0.0, 10.0, -10.0), VColor::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(light, position, eyev, normalv);

        let expected_result = VColor::new(0.7364, 0.7364, 0.7364);

        assert_zeq!(actual_result, expected_result);
    }

    #[test]
    fn lighting_with_the_eye_in_path_of_the_reflection_vector() {
        let m = VPhong::default();
        //let body = VBody::from(VSphere::default());
        let position = VTuple::point(0.0, 0.0, 0.0);

        let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
        let eyev = VTuple::vector(0.0, -sqrt2_over_2, -sqrt2_over_2);
        let normalv = VTuple::vector(0.0, 0.0, -1.0);
        let light = VPointLight::new(VTuple::point(0.0, 10.0, -10.0), VColor::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting( light, position, eyev, normalv);

        let expected_result = VColor::new(1.6364, 1.6364, 1.6364);

        assert_zeq!(actual_result, expected_result);
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = VPhong::default();
        //let body = VBody::from(VSphere::default());
        let position = VTuple::point(0.0, 0.0, 0.0);

        let eyev = VTuple::vector(0.0, 0.0, -1.0);
        let normalv = VTuple::vector(0.0, 0.0, -1.0);
        let light = VPointLight::new(VTuple::point(0.0, 0.0, 10.0), VColor::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting( light, position, eyev, normalv);

        let expected_result = VColor::new(0.1, 0.1, 0.1);

        assert_zeq!(actual_result, expected_result);
    }

    // #[test]
    // fn lighting_with_the_surface_in_shadow() {
    //     let m = VPhong::default();
    //     let body = VBody::from(VSphere::default());
    //     let position = VTuple::point(0.0, 0.0, 0.0);

    //     let eyev = VTuple::vector(0.0, 0.0, -1.0);
    //     let normalv = VTuple::vector(0.0, 0.0, -1.0);
    //     let light = VPointLight::new(VTuple::point(0.0, 0.0, -10.0), VColor::new(1.0, 1.0, 1.0));

    //     let actual_result = m.lighting( light, position, eyev, normalv);

    //     let expected_result = VColor::new(0.1, 0.1, 0.1);

    //     assert_zeq!(actual_result, expected_result);
    // }

//     #[test]
//     fn phong_material_has_reflective_zero_by_default() {
//         let m = VPhong::default();

//         assert_zeq!(0.0, m.reflectiveness);
//     }

//     #[test]
//     fn phong_material_has_builder_function_for_reflectiveness() {
//         let m = VPhong::default().with_reflectiveness(0.42);

//         assert_zeq!(0.42, m.reflectiveness);
//     }
}
