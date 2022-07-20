use std::ops;

use crate::body::VBody;
use crate::{ray::VRay, F};

#[derive(Debug)]
pub struct VIntersection {
    pub t: F,
    pub ray: VRay,
    pub body: VBody,
}

impl VIntersection {
    pub fn new(t: F, ray: VRay, body: VBody) -> VIntersection {
        VIntersection { t, ray, body }
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
