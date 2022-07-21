use crate::body::*;
use crate::canvas::vcolor::VColor;
use crate::intersections::*;
use crate::material::Illuminated;
use crate::ray::*;
use crate::{body::VBody, light::VPointLight};
// use crate::canvas::vcolor::*;
// use crate::zequality::*;

pub struct VWorld {
    pub bodies: Vec<VBody>,
    pub lights: Vec<VPointLight>,
}
impl VWorld {
    pub fn new(bodies: Vec<VBody>, lights: Vec<VPointLight>) -> Self {
        VWorld { bodies, lights }
    }
    pub fn intersect(&self, ray: VRay) -> VIntersections {
        let xs = self
            .bodies
            .iter()
            .flat_map(|body| body.intersect(ray))
            .collect();
        VIntersections::new(xs)
    }
    pub fn color_at(&self, ray: VRay) -> VColor {
        let xs = self.intersect(ray);
        let hit = xs.hit();
        if let Some(hit) = hit {
            let c = hit.get_computed();
            let material = hit.body.material();

            //implement proper lighting to allow multiple lights
            material.lighting(self.lights[0], c.pos, c.camv, c.normalv)
        } else {
            VColor::black()
        }
    }
    // pub fn color_at(&self, ray: VRay) -> VColor {
    //     self.color_at_with_reflection_limit(ray, self.reflection_limit)
    // }
    // fn color_at_with_reflection_limit(&self, ray: VRay, remaining_reflections: usize) -> Color {
    //     let xs = self.intersect(ray);
    //     let hit = xs.hit();
    //     if let Some(hit) = hit {
    //         let c = hit.get_computed();
    //         let material = hit.body.material();
    //         // @TODO: Implement proper lighting using multiple light sources
    //         let is_in_shadow = self.is_shadowed(c.over_point);
    //         let surface_color = material.lighting(
    //             &hit.body,
    //             self.lights[0],
    //             c.over_point,
    //             c.eyev,
    //             c.normalv,
    //             is_in_shadow,
    //         );

    //         let reflected_color = self.reflected_color_at(&material, &c, remaining_reflections);

    //         surface_color + reflected_color
    //     } else {
    //         Color::black()
    //     }
    // }
}
impl Default for VWorld {
    fn default() -> Self {
        VWorld {
            bodies: vec![],
            lights: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::vcolor::*;
    use crate::material::*;
    use crate::matrix::VMatrix;
    use crate::sphere::*;
    use crate::tuple::VTuple;
    use crate::zequality::*;

    fn create_default_world() -> VWorld {
        let light = VPointLight::new(
            VTuple::point(-10.0, 10.0, -10.0),
            VColor::new(1.0, 1.0, 1.0),
        );
        // @FIXME: Rafactor to use new builder pattern.
        let material = VPhong {
            col: VColor::new(0.8, 1.0, 0.6),
            dif: 0.7,
            spc: 0.2,
            ..VPhong::default()
        };
        let s1 = VBody::from(VSphere::default().with_material(VMaterial::from(material)));
        let s2 = VBody::from(VSphere::default().with_transform(VMatrix::scaling(0.5, 0.5, 0.5)));
        VWorld::new(vec![s1, s2], vec![light])
    }

    #[test]
    fn the_default_world() {
        let light = VPointLight::new(
            VTuple::point(-10.0, 10.0, -10.0),
            VColor::new(1.0, 1.0, 1.0),
        );
        let material = VPhong {
            col: VColor::new(0.8, 1.0, 0.6),
            dif: 0.7,
            spc: 0.2,
            ..VPhong::default()
        };
        let s1 = VBody::from(VSphere::default().with_material(VMaterial::from(material)));
        let s2 = VBody::from(VSphere::default().with_transform(VMatrix::scaling(0.5, 0.5, 0.5)));
        let world = create_default_world();

        assert_eq!(2, world.bodies.len());
        assert_eq!(1, world.lights.len());
        assert!(world.bodies.contains(&s1));
        assert!(world.bodies.contains(&s2));
        assert!(world.lights.contains(&light));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = create_default_world();
        let r = VRay::new(VTuple::point(0.0, 0.0, -5.0), VTuple::vector(0.0, 0.0, 1.0));
        let xs = w.intersect(r);

        assert_eq!(4, xs.len());
        assert_zeq!(4.0, xs[0].t);
        assert_zeq!(4.5, xs[1].t);
        assert_zeq!(5.5, xs[2].t);
        assert_zeq!(6.0, xs[3].t);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = create_default_world();
        let r = VRay::new(VTuple::point(0.0, 0.0, -5.0), VTuple::vector(0.0, 1.0, 0.0));
        let c = w.color_at(r);

        assert_zeq!(c, VColor::black());
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = create_default_world();
        let r = VRay::new(VTuple::point(0.0, 0.0, -5.0), VTuple::vector(0.0, 0.0, 1.0));
        let c = w.color_at(r);

        assert_zeq!(c, VColor::new(0.38066, 0.47583, 0.2855));
    }

    //   #[test]
    //   fn there_is_no_shadow_when_nothing_is_colinear_with_point_and_light() {
    //     let w = create_default_world();
    //     let p = Tuple::point(0.0, 10.0, 0.0);
    //     let is_in_shadow = w.is_shadowed(p);

    //     assert_eq!(is_in_shadow, false);
    //   }

    //   #[test]
    //   fn there_is_shadow_when_an_object_is_between_the_point_and_the_light() {
    //     let w = create_default_world();
    //     let p = Tuple::point(10.0, -10.0, 10.0);
    //     let is_in_shadow = w.is_shadowed(p);

    //     assert_eq!(is_in_shadow, true);
    //   }

    //   #[test]
    //   fn there_is_no_shadow_when_an_object_is_behind_the_light() {
    //     let w = create_default_world();
    //     let p = Tuple::point(-20.0, 20.0, -20.0);
    //     let is_in_shadow = w.is_shadowed(p);

    //     assert_eq!(is_in_shadow, false);
    //   }

    //   #[test]
    //   fn there_is_no_shadow_when_an_object_is_behind_the_point() {
    //     let w = create_default_world();
    //     let p = Tuple::point(-2.0, 2.0, -2.0);
    //     let is_in_shadow = w.is_shadowed(p);

    //     assert_eq!(is_in_shadow, false);
    //   }

    //   #[test]
    //   fn the_color_when_a_ray_hits_something_in_shadow() {
    //     let material = Material::default();
    //     let s1 = Sphere::new(material, Matrix::identity());
    //     let s2 = Sphere::new(material, Matrix::translation(0.0, 0.0, 10.0));
    //     let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    //     let w = World::new(vec![s1.into(), s2.into()], vec![light]);

    //     let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let c = w.color_at(r);

    //     assert_fuzzy_eq!(c, Color::new(0.1, 0.1, 0.1));
    //   }

    //   #[test]
    //   fn reflection_color_if_non_reflective_body_is_hit() {
    //     let non_reflective_material = Material::from(
    //       Phong::default()
    //         .with_color(Color::new(0.8, 1.0, 0.6))
    //         .with_ambient(1.0)
    //         .with_reflectiveness(0.0),
    //     );
    //     let s1 = Body::from(Sphere::default().with_material(non_reflective_material));
    //     let world = World::new(vec![s1], vec![]);
    //     let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

    //     let intersection = Intersection::new(1.0, ray, s1);
    //     let reflected_color = world.reflected_color_at(
    //       &intersection.body.material(),
    //       &intersection.get_computed(),
    //       1,
    //     );

    //     assert_fuzzy_eq!(reflected_color, Color::black());
    //   }

    //   #[test]
    //   fn reflection_color_if_reflective_body_is_hit() {
    //     let non_reflective_material = Material::from(
    //       Phong::default()
    //         .with_color(Color::new(0.5, 0.25, 0.125))
    //         .with_ambient(1.0)
    //         .with_reflectiveness(0.5),
    //     );
    //     let s1 = Body::from(Sphere::default().with_material(non_reflective_material));
    //     let world = World::new(
    //       vec![s1],
    //       vec![PointLight::new(
    //         Tuple::point(10.0, 10.0, 10.0),
    //         Color::white(),
    //       )],
    //     );
    //     let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));

    //     let intersection = Intersection::new(1.0, ray, s1);
    //     let reflected_color = world.reflected_color_at(
    //       &intersection.body.material(),
    //       &intersection.get_computed(),
    //       2,
    //     );

    //     assert_fuzzy_eq!(reflected_color, Color::new(0.375, 0.1875, 0.09375));
    //   }
}
