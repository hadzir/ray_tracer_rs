use std::ops;

use crate::body::*;
use crate::{ray::VRay, F};

use crate::tuple::VTuple;

#[derive(Debug, Clone)]
pub struct VComputedIntersection<'a> {
    pub intersection: &'a VIntersection,
    pub pos: VTuple,
    pub point: VTuple,
    pub normalv: VTuple,
    pub camv: VTuple,
    pub reflectv: VTuple,
    pub inside: bool,
}

impl<'a> VComputedIntersection<'a> {
    pub fn new(
        intersection: &'a VIntersection,
        pos: VTuple,
        point: VTuple,
        normalv: VTuple,
        camv: VTuple,
        reflectv: VTuple,
        inside: bool,
    ) -> Self {
        VComputedIntersection {
            intersection,
            pos,
            point,
            normalv,
            camv,
            reflectv,
            inside,
        }
    }
}

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

//   impl ops::IntoIterator for VIntersections {
//     type Item = VIntersection;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//       self.data.into_iter()
//     }
//   }
