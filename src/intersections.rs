use std::ops;

use crate::body::*;
use crate::computed_intersection::*;
use crate::{ray::VRay, F};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VIntersection {
    pub t: F,
    pub ray: VRay,
    pub body: VBody,
}

impl VIntersection {
    pub fn new(t: F, ray: VRay, body: VBody) -> VIntersection {
        VIntersection { t, ray, body }
    }

    pub fn get_computed(&self) -> VComputedIntersection {
        let position = self.ray.position(self.t);
        let mut normalv = self.body.normal_at(position);
        let eyev = -self.ray.direction;
        let inside = normalv.dot(&eyev) < 0.0;

        if inside {
            normalv = -normalv;
        }
        let over_point = position + normalv * 0.0001;
        let reflectv = self.ray.direction.reflected(normalv);

        VComputedIntersection::new(self, position, over_point, normalv, eyev, reflectv, inside)
    }
}

pub struct VIntersections {
    data: Vec<VIntersection>,
}

impl VIntersections {
    pub fn new(mut intersections: Vec<VIntersection>) -> Self {
        intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        VIntersections {
            data: intersections,
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn hit(&self) -> Option<&VIntersection> {
        for intersection in self.data.iter() {
            if intersection.t > 0.0 {
                return Some(intersection);
            }
        }

        None
    }
}
impl From<Vec<VIntersection>> for VIntersections {
    fn from(v: Vec<VIntersection>) -> Self {
        Self::new(v)
    }
}

impl ops::Index<usize> for VIntersections {
    type Output = VIntersection;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IntoIterator for VIntersections {
    type Item = VIntersection;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::*;
    use crate::tuple::*;
    use crate::zequality::*;

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = VSphere::default();
        let r = VRay::new(VTuple::point(1.0, 1.0, 1.0), VTuple::vector(0.0, 0.0, 1.0));
        let i1 = VIntersection::new(1.0, r, VBody::from(s));
        let i2 = VIntersection::new(2.0, r, VBody::from(s));
        let xs = VIntersections::new(vec![i2, i1]);

        assert_eq!(xs.hit(), Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = VSphere::default();
        let r = VRay::new(VTuple::point(1.0, 1.0, 1.0), VTuple::vector(0.0, 0.0, 1.0));
        let i1 = VIntersection::new(-1.0, r, VBody::from(s));
        let i2 = VIntersection::new(1.0, r, VBody::from(s));
        let xs = VIntersections::new(vec![i2, i1]);

        assert_eq!(xs.hit(), Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = VSphere::default();
        let r = VRay::new(VTuple::point(1.0, 1.0, 1.0), VTuple::vector(0.0, 0.0, 1.0));
        let i1 = VIntersection::new(-2.0, r, VBody::from(s));
        let i2 = VIntersection::new(-1.0, r, VBody::from(s));
        let xs = VIntersections::new(vec![i2, i1]);

        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = VRay::new(VTuple::point(0.0, 0.0, -5.0), VTuple::vector(0.0, 0.0, 1.0));
        let body = VBody::from(VSphere::default());
        let i = VIntersection::new(4.0, r, body);
        let c = i.get_computed();

        assert_eq!(c.intersection, &i);
        assert_zeq!(c.pos, VTuple::point(0.0, 0.0, -1.0));
        assert_zeq!(c.camv, VTuple::vector(0.0, 0.0, -1.0));
        assert_zeq!(c.normalv, VTuple::vector(0.0, 0.0, -1.0));
    }

    // #[test]
    // fn precomputing_reflection_vector() {
    //     let body = VBody::from(VPlane::default());
    //     let r = VRay::new(
    //         VTuple::point(0.0, 1.0, -1.0),
    //         VTuple::vector(0.0, -(2.0 as F).sqrt() / 2.0, (2.0 as F).sqrt() / 2.0),
    //     );
    //     let intersection = VIntersection::new((2.0 as F).sqrt(), r, body);
    //     let computations = intersection.get_computed();

    //     assert_zeq!(
    //         computations.reflectv,
    //         VTuple::vector(0.0, (2.0 as F).sqrt() / 2.0, (2.0 as F).sqrt() / 2.0)
    //     );
    // }

    // #[test]
    // fn the_hit_when_an_intersection_occurs_on_the_outside() {
    //     let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let body = Body::from(Sphere::default());
    //     let i = Intersection::new(4.0, r, body);
    //     let c = i.get_computed();

    //     assert_eq!(c.inside, false);
    // }

    // #[test]
    // fn the_hit_when_an_intersection_occurs_on_the_inside() {
    //     let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let body = Body::from(Sphere::default());
    //     let i = Intersection::new(1.0, r, body);
    //     let c = i.get_computed();

    //     assert_eq!(c.inside, true);
    //     assert_eq!(c.normalv, Tuple::vector(0.0, 0.0, -1.0));
    // }

    // #[test]
    // fn the_hit_should_offset_the_point() {
    //     let material = Material::default();
    //     let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let s1 = Sphere::new(material, Matrix::translation(0.0, 0.0, 1.0));
    //     let i = Intersection::new(5.0, r, s1.into());
    //     let c = i.get_computed();

    //     assert!(c.over_point.z < -EPSILON / 2.0);
    //     assert!(c.point.z > c.over_point.z);
    // }
}
