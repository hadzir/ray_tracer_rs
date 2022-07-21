use crate::intersections::*;
use crate::tuple::*;

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