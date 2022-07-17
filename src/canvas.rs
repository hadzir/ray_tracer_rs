use std::vec::Vec;
use crate::VColor;

pub struct VCanvas {
    pub width: usize,
    pub height: usize,

    pixels: Vec<VColor>
}
impl VCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, pixels:vec![VColor::black();width*height]}
    }
    fn get_pixel_index(&self,x:usize,y:usize)->usize{
        y*self.width + x
    }
    pub fn write_color(&mut self,x:usize,y:usize,color:VColor){
        let index = self.get_pixel_index(x,y);
        self.pixels[index] = color;
    }
    pub fn pixel_at(&self,x:usize,y:usize) -> &VColor{
        &self.pixels[self.get_pixel_index(x,y)]
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

        for x in 0..cnv.width-1{
            for y in 0..cnv.height-1{
                assert_eq!(VColor::black(),*cnv.pixel_at(x,y))
            }
        }
    }
    #[test]
    fn writing_pixels_to_a_canvas(){
        let mut cnv = VCanvas::new(10, 20);

        let c=VColor::red();
        cnv.write_color(2,3,c);

        assert_eq!(VColor::red(),*cnv.pixel_at(2,3))
    }
}
