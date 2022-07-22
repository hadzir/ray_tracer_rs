use serde::Deserialize;

use crate::{matrix::VMatrix, ray::VRay, tuple::VTuple, F};

#[derive(Debug,Clone, Copy,PartialEq,Deserialize)]
pub struct VCamera {
    pub transform: VMatrix<4>,
    pub vsize: usize,
    pub hsize: usize,
    pub fov: F,
    half_width: F,
    half_height: F,
    pixel_size: F,
}
impl VCamera {
    pub fn new(hsize: usize, vsize: usize, fov: F) -> Self {
        let half_width;
        let half_height;

        let half_size = (fov / 2.0).tan();
        let aspect_ratio = hsize as F / vsize as F;
        if aspect_ratio >= 1.0 {
            //Portrait mode
            half_width = half_size;
            half_height = half_size / aspect_ratio;
        } else {
            half_height = half_size;
            half_width = half_size * aspect_ratio;
        }
        let pixel_size = (half_width * 2.0) / hsize as F;
        Self {
            transform: VMatrix::identity(),
            vsize,
            hsize,
            fov,
            half_width,
            half_height,
            pixel_size,
        }
    }
    pub fn with_transform(mut self, transform: VMatrix<4>) -> Self {
        self.transform = transform;
        self
    }
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> VRay {
        let offset_x = (0.5 + x as f64) * self.pixel_size;
        let offset_y = (0.5 + y as f64) * self.pixel_size;

        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        let ivt = self.transform.inverted();

        let wall_point = ivt * VTuple::point(world_x, world_y, -1.0);
        let o = ivt * VTuple::point(0.0, 0.0, 0.0);

        let d = (wall_point - o).normalized();

        VRay::new(o, d)
    }
    pub fn position_and_point(mut self, from: VTuple, to: VTuple, up: VTuple) {
        let forwardv = (to - from).normalized();
        let leftv = forwardv.crossed(&up);
        let true_upv = leftv.crossed(&forwardv);
        #[rustfmt::skip]
        let orientation_transform  =VMatrix::from([
            [leftv.x,       leftv.y,    leftv.z,    0.0],
            [true_upv.x,    true_upv.y, true_upv.z, 0.0],
            [forwardv.x,    forwardv.y, forwardv.z, 0.0],
            [0.0,           0.0,        0.0,        1.0],
            ]);
        let translation_transform = VMatrix::translation(-from.x, -from.y, -from.z);

        self.transform = orientation_transform*translation_transform;
    }
    pub fn positioned_and_pointed(mut self, from: VTuple, to: VTuple, up: VTuple) -> Self {
        let forwardv = (to - from).normalized();
        let leftv = forwardv.crossed(&up);
        let true_upv = leftv.crossed(&forwardv);
        #[rustfmt::skip]
        let orientation_transform  =VMatrix::from([
            [leftv.x,       leftv.y,    leftv.z,    0.0],
            [true_upv.x,    true_upv.y, true_upv.z, 0.0],
            [-forwardv.x,  -forwardv.y,-forwardv.z, 0.0],
            [0.0,           0.0,        0.0,        1.0],
            ]);
        let translation_transform = VMatrix::translation(-from.x, -from.y, -from.z);

        self.transform = orientation_transform*translation_transform;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::zequality::*;
    use std::f64::consts::PI;

    use super::*;
    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI / 2.0;
        let cam = VCamera::new(hsize, vsize, fov);

        assert_eq!(cam.vsize, vsize);
        assert_eq!(cam.hsize, hsize);
        assert_zeq!(cam.fov, fov);
    }

    #[test]
    fn constructed_camera_has_identity_transform() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI / 2.0;
        let cam = VCamera::new(vsize, hsize, fov);
        assert_zeq!(cam.transform, VMatrix::identity());
    }

    #[test]
    fn constructed_camera_can_be_transformed() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI / 2.0;
        let t = VMatrix::translation(1.0, 2.0, 3.0);
        let cam = VCamera::new(vsize, hsize, fov).with_transform(t);

        assert_zeq!(cam.transform, t);
    }
    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = VCamera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_zeq!(r.origin, VTuple::point(0.0, 0.0, 0.0));
        assert_zeq!(r.direction, VTuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_the_corner_of_a_canvas() {
        let c = VCamera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_zeq!(r.origin, VTuple::point(0.0, 0.0, 0.0));
        assert_zeq!(r.direction, VTuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_camera_is_transformed() {
        let c = VCamera::new(201, 101, PI / 2.0)
            .with_transform(VMatrix::rotation_y(PI / 4.0) * VMatrix::translation(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100, 50);

        assert_zeq!(r.origin, VTuple::point(0.0, 2.0, -5.0));
        assert_zeq!(
            r.direction,
            VTuple::vector((2.0 as f64).sqrt() / 2.0, 0.0, -((2.0 as f64).sqrt()) / 2.0)
        );
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = VCamera::new(200, 125, PI / 2.0);
        assert_zeq!(c.pixel_size, 0.01);
    }

    #[test]
    fn pixel_size_for_vertical() {
        let c = VCamera::new(125, 200, PI / 2.0);
        assert_zeq!(c.pixel_size, 0.01);
    }
}
