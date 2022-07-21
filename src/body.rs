use crate::{
    intersections::{VIntersection, VIntersections},
    matrix::VMatrix,
    ray::VRay,
    sphere::VSphere,
    tuple::VTuple,
    F, material::VMaterial,
};

pub trait VIntersectable {
    fn material(&self) -> VMaterial;
    fn transform(&self) -> VMatrix<4>;
    fn normal_at_in_object_space(&self, object_space_point: VTuple) -> VTuple;

    fn intersect_in_object_space(&self, object_space_ray: VRay) -> Vec<(F, VBody)>;

    fn intersect(&self, ray: VRay) -> VIntersections {
        let object_space_ray = ray.transformed(self.transform().inverted());
        let ts = self.intersect_in_object_space(object_space_ray);
        VIntersections::new(
            ts.into_iter()
                .map(|(t, body)| VIntersection::new(t, ray, body))
                .collect(),
        )
    }
    fn normal_at(&self, point: VTuple) -> VTuple {
        let object_space_point = self.transform().inverted() * point;

        let object_normal = self.normal_at_in_object_space(object_space_point);

        let mut world_normal = self.transform().inverted().transposed() * object_normal;
        // Hack, to ensure we have a clean vector, as due the inverse transpose the
        // w component could be affected if the transformation matrix included a
        // translation
        world_normal.w = 0.0;
        world_normal.normalized()
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VBody {
    VSphere(VSphere),
}
impl VIntersectable for VBody {
    fn intersect_in_object_space(&self, object_space_ray: VRay) -> Vec<(F, VBody)> {
        match *self {
            VBody::VSphere(ref sphere) => sphere.intersect_in_object_space(object_space_ray),
            //VBody::Plane(ref plane) => plane.intersect_in_object_space(object_space_ray),
        }
    }
    fn transform(&self) -> VMatrix<4> {
        match *self {
            VBody::VSphere(ref sphere) => sphere.transform(),
            //VBody::VPlane(ref plane) => plane.transform(),
        }
    }
    fn normal_at_in_object_space(&self, object_space_point: VTuple) -> VTuple {
        match *self {
            VBody::VSphere(ref sphere) => sphere.normal_at_in_object_space(object_space_point),
            //VBody::VPlane(ref plane) => plane.transform(),
        }
    }
    fn material(&self)->VMaterial{
        match *self {
            VBody::VSphere(ref sphere)=>{sphere.material()}            
        }
    }
}
impl From<VSphere> for VBody {
    fn from(sphere: VSphere) -> Self {
        VBody::VSphere(sphere)
    }
}
