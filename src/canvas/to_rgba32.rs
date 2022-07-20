use num_traits::Float;

use crate::canvas::vcanvas::VCanvas;

pub trait ToRGBA32 {
    fn to_rgba32(&self) -> Vec<u8>;
}

impl<T> ToRGBA32 for VCanvas<T>
where
    T: Float,
{
    fn to_rgba32(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        for pixel in self.get_pixels().iter() {
            let clamped_color = pixel.clamp(T::zero(), T::one());
            let r: u8 = (clamped_color.r.to_f64().unwrap() * 255.0).round() as u8;
            let g: u8 = (clamped_color.g.to_f64().unwrap() * 255.0).round() as u8;
            let b: u8 = (clamped_color.b.to_f64().unwrap() * 255.0).round() as u8;
            let a: u8 = 255;

            data.push(r);
            data.push(g);
            data.push(b);
            data.push(a);
        }
        data
    }
}
