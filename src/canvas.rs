use crate::color::*;
use std::vec::Vec;

pub struct VCanvas {
    pub width: usize,
    pub height: usize,

    pixels: Vec<VColor>,
}
impl VCanvas {
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
    pub fn write_pixel(&mut self, x: usize, y: usize, color: VColor) {
        let index = self.get_pixel_index(x, y);
        self.pixels[index] = color;
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> VColor {
        self.pixels[self.get_pixel_index(x, y)]
    }
}

pub trait ToPPM {
    #[doc(hidden)]
    fn create_ppm_header(&self, width: usize, height: usize, max_color_value: usize) -> Vec<u8> {
        let mut header = Vec::new();
        header.extend(String::from("P3\n").into_bytes());
        header.extend(format!("{} {}\n", width, height).into_bytes());
        header.extend(format!("{}\n", max_color_value).into_bytes());
        return header;
    }
    fn to_ppm(&self) -> Vec<u8>;
}

impl ToPPM for VCanvas {
    fn to_ppm(&self) -> Vec<u8> {
        let mut byte_array = self.create_ppm_header(self.width, self.height, 255);

        for y in 0..self.height {
            let mut row_string = String::new();
            for x in 0..self.width {
                let pixel = self.pixel_at(x, y);
                row_string += &pixel.to_rgb_str();

                if (x + 1) < self.width {
                    row_string += " ";
                };
            }
            row_string += "\n";
            byte_array.extend(row_string.into_bytes());
        }
        return byte_array;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creating_a_canvas() {
        let cnv = VCanvas::new(10, 20);

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
        let mut cnv = VCanvas::new(10, 20);

        let c = VColor::red();
        cnv.write_pixel(2, 3, c);

        assert_eq!(VColor::red(), cnv.pixel_at(2, 3))
    }
    #[test]
    fn constructing_the_ppm_header() {
        let cnv = VCanvas::new(5, 3);

        let result = cnv.create_ppm_header(cnv.width, cnv.height, 255);
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
