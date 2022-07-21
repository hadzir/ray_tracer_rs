use crate::body::VBody;
use crate::body::VIntersectable;
use crate::material::*;
use crate::matrix::*;
use crate::ray::VRay;
use crate::tuple::VTuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VSphere {
    pub transform: VMatrix<4>,
    pub material: VMaterial,
}

impl Default for VSphere {
    fn default() -> Self {
        Self {
            transform: VMatrix::identity(),
            material: Default::default(),
        }
    }
}
impl VSphere {
    pub fn new(transform: Option<VMatrix<4>>, material: VMaterial) -> Self {
        match transform {
            Some(transform) => VSphere {
                transform,
                material,
            },
            None => VSphere::default(),
        }
    }

    pub fn with_transform(mut self, transform: VMatrix<4>) -> Self {
        self.transform = transform;
        self
    }
    pub fn with_material(mut self, material: VMaterial) -> Self {
        self.material = material;
        self
    }
}
impl VIntersectable for VSphere {
    fn intersect_in_object_space(&self, ray: VRay) -> Vec<(f64, VBody)> {
        let sphere_to_ray = ray.origin - VTuple::point(0.0, 0.0, 0.0);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                ((-b - discriminant.sqrt()) / (2.0 * a), VBody::from(*self)),
                ((-b + discriminant.sqrt()) / (2.0 * a), VBody::from(*self)),
            ]
        }
    }

    fn transform(&self) -> VMatrix<4> {
        self.transform
    }
    fn normal_at_in_object_space(&self, object_space_point: VTuple) -> VTuple {
        (object_space_point - VTuple::point(0.0, 0.0, 0.0)).normalize()
    }
    fn material(&self) -> VMaterial {
        self.material
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{zequality::ZEq, F};
    use std::f64::consts::PI;
    use crate::canvas::vcolor::VColor;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let o = VTuple::point(0.0, 0.0, -5.0);
        let d = VTuple::vector(0.0, 0.0, 1.0);
        let r = VRay::new(o, d);
        let s = VSphere::default();
        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_zeq!(4.0, xs[0].t);
        assert_zeq!(6.0, xs[1].t);
    }
    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let o = VTuple::point(0.0, 1.0, -5.0);
        let d = VTuple::vector(0.0, 0.0, 1.0);
        let r = VRay::new(o, d);
        let s = VSphere::default();
        let xs = s.intersect(r);

        assert_zeq!(5.0, xs[0].t);
        assert_zeq!(5.0, xs[1].t);
    }
    #[test]
    fn a_ray_misses_a_sphere() {
        let o = VTuple::point(0.0, 2.0, -5.0);
        let d = VTuple::vector(0.0, 0.0, 1.0);
        let r = VRay::new(o, d);
        let s = VSphere::default();
        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let o = VTuple::point(0.0, 0.0, 0.0);
        let d = VTuple::vector(0.0, 0.0, 1.0);
        let r = VRay::new(o, d);
        let s = VSphere::default();
        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }
    #[test]
    fn a_sphere_is_behind_a_ray() {
        let o = VTuple::point(0.0, 0.0, 5.0);
        let d = VTuple::vector(0.0, 0.0, 1.0);
        let r = VRay::new(o, d);
        let s = VSphere::default();
        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }
    #[test]
    fn a_spheres_default_transform() {
        let s = VSphere::default();
        assert_zeq!(s.transform, VMatrix::identity());
    }

    #[test]
    fn changing_a_spheres_transform() {
        let m = VMatrix::translation(2.0, 3.0, 4.0);
        let s = VSphere::default().with_transform(m);

        assert_zeq!(s.transform, m);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let o = VTuple::point(0.0, 1.5, -5.0);
        let d = VTuple::vector(0.0, 0.0, 1.0);
        let r = VRay::new(o, d);
        let s = VSphere::default().with_transform(VMatrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let o = VTuple::point(0.0, 0.0, -5.0);
        let d = VTuple::vector(0.0, 0.0, 1.0);
        let r = VRay::new(o, d);
        let s = VSphere::default().with_transform(VMatrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = VSphere::default();
        let n = s.normal_at(VTuple::point(1.0, 0.0, 0.0));

        let expected_result = VTuple::vector(1.0, 0.0, 0.0);

        assert_zeq!(n, expected_result);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = VSphere::default();
        let n = s.normal_at(VTuple::point(0.0, 1.0, 0.0));

        let expected_result = VTuple::vector(0.0, 1.0, 0.0);

        assert_zeq!(n, expected_result);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = VSphere::default();
        let n = s.normal_at(VTuple::point(0.0, 0.0, 1.0));

        let expected_result = VTuple::vector(0.0, 0.0, 1.0);

        assert_zeq!(n, expected_result);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s = VSphere::default();
        let sqrt3_over_3 = (3.0 as F).sqrt() / 3.0;
        let p = VTuple::point(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3);
        let n = s.normal_at(p);

        let expected_result = VTuple::vector(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3);

        assert_zeq!(n, expected_result);
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = VSphere::default().with_transform(VMatrix::translation(0.0, 1.0, 0.0));
        let p = VTuple::point(0.0, 1.70711, -0.70711);
        let n = s.normal_at(p);

        let expected_result = VTuple::vector(0.0, 0.70711, -0.70711);

        assert_zeq!(n, expected_result);
    }

    #[test]
    fn computing_the_normal_on_a_scaled_and_rotated_sphere() {
        let s = VSphere::default()
            .with_transform(VMatrix::scaling(1.0, 0.5, 1.0) * VMatrix::rotation_z(PI / 5.0));
        let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
        let p = VTuple::point(0.0, sqrt2_over_2, -sqrt2_over_2);
        let n = s.normal_at(p);

        let expected_result = VTuple::vector(0.0, 0.97014, -0.24254);

        assert_zeq!(n, expected_result);
    }

    #[test]
    fn the_normal_vector_is_always_normalized() {
        let s = VSphere::default();
        let sqrt3_over_3 = (3.0 as F).sqrt() / 3.0;
        let p = VTuple::point(sqrt3_over_3, sqrt3_over_3, sqrt3_over_3);
        let n = s.normal_at(p);

        assert_zeq!(n.normalize(), n);
    }

    #[test]
    fn the_normal_vector_is_normalized_on_transformed_sphere() {
        let s = VSphere::default()
            .with_transform(VMatrix::scaling(1.0, 0.5, 1.0) * VMatrix::rotation_z(PI / 5.0));
        let sqrt2_over_2 = (2.0 as F).sqrt() / 2.0;
        let p = VTuple::point(0.0, sqrt2_over_2, -sqrt2_over_2);
        let n = s.normal_at(p);

        assert_zeq!(n.normalize(), n);
    }

    #[test]
    fn sphere_has_default_phong_material() {
        let s = VSphere::default();
        let m = VMaterial::default();

        assert_zeq!(s.material, m);
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let phong = VPhong::default()
            .with_color(VColor::new(1.0, 1.0, 0.0))
            .with_ambient(0.05)
            .with_diffuse(0.7)
            .with_specular(0.95)
            .with_shininess(400.0);
        let m = VMaterial::from(phong);
        let s = VSphere::default().with_material(m);

        assert_zeq!(s.material, m);
    }
}
