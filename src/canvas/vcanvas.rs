use num_traits::Float;

use crate::canvas::vcolor::*;
use std::vec::Vec;
pub trait Sized {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}
impl<T: Float> Sized for VCanvas<T> {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}
pub struct VCanvas<T = f64>
where
    T: Float,
{
    pub width: usize,
    pub height: usize,

    pixels: Vec<VColor<T>>,
}
impl<T> VCanvas<T>
where
    T: Float,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![VColor::black(); width * height],
        }
    }
    fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: VColor<T>) {
        let index = self.get_pixel_index(x, y);
        self.pixels[index] = color;
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> VColor<T> {
        self.pixels[self.get_pixel_index(x, y)]
    }
    pub fn get_pixels(&self) -> &Vec<VColor<T>> {
        &self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::to_ppm::ToPPM;

    #[test]
    fn creating_a_canvas() {
        let cnv : VCanvas<f64> = VCanvas::new(10, 20);

        assert_eq!(cnv.width, 10);
        assert_eq!(cnv.height, 20);

        for x in 0..cnv.width - 1 {
            for y in 0..cnv.height - 1 {
                assert_eq!(VColor::black(), cnv.pixel_at(x, y))
            }
        }
    }
    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut cnv: VCanvas<f64> = VCanvas::new(10, 20);

        let c = VColor::red();
        cnv.write_pixel(2, 3, c);

        assert_eq!(VColor::red(), cnv.pixel_at(2, 3))
    }
    #[test]
    fn constructing_the_ppm_header() {
        let cnv: VCanvas<f64>  = VCanvas::new(5, 3);

        let result = cnv.create_ppm_header();
        let expected_result = String::from("P3\n5 3\n255\n").into_bytes();

        assert_eq!(result, expected_result)
    }
    #[test]
    fn constructing_the_pixel_data() {
        let mut cnv = VCanvas::new(2, 3);
        let c1 = VColor::new(1.5, 0.0, 0.0);
        let c2 = VColor::new(0.0, 0.5, 0.0);
        let c3 = VColor::new(-0.5, 0.0, 1.0);

        cnv.write_pixel(0, 0, c1);
        cnv.write_pixel(0, 1, c2);
        cnv.write_pixel(0, 2, c3);

        let result = cnv.to_ppm();
        let expected_result =
            String::from("P3\n2 3\n255\n255 0 0 0 0 0\n0 128 0 0 0 0\n0 0 255 0 0 0\n")
                .into_bytes();

        assert_eq!(result, expected_result)
    }
}
