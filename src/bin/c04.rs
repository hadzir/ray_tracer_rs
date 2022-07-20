use core::panic;
use std::f64::consts::PI;
use std::fs::write;
use ray_tracer::F;
use ray_tracer::canvas::to_png::*;
use ray_tracer::canvas::vcanvas::*;
use ray_tracer::canvas::vcolor::*;
use ray_tracer::matrix::VMatrix;
use ray_tracer::tuple::*;

fn main() {
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;
    const RADIUS: f64 = 450.0;
    let mut cnv: VCanvas = VCanvas::new(WIDTH, HEIGHT);

    let new_origin = VTuple::point((WIDTH / 2) as f64, (HEIGHT / 2) as f64, 0.0);
    let t_org = VMatrix::translation(new_origin.x, new_origin.y, new_origin.z);



    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let p = VTuple::point(x as f64, y as f64, 0.0);
            let d = (p - new_origin).magnitude();
            if d < RADIUS {
                match Pixel::from_point_to_canvas(p, &cnv) {
                    Pixel::Coordinate { x, y } => cnv.write_pixel(x, y, VColor::red()),
                    Pixel::OutOfBounds { x, y } => {
                        println!("Coordinates x:{}, y:{} could not be mapped to screen", x, y)
                    }
                }
            }
        }
    }
    for ang in 0..360 {
        let mut r=1.0;
        while r<RADIUS{
            r+=1.0;
            let t_rot = VMatrix::rotation_z(PI / 180.0 * ang as f64);
            let point = VTuple::point(0.0, r, 0.0);
    
            let p = t_org * t_rot * point;
    
            match Pixel::from_point_to_canvas(p, &cnv) {
                Pixel::Coordinate { x, y } => cnv.write_pixel(x, y, VColor::blue()),
                Pixel::OutOfBounds { x, y } => {
                    println!("Coordinates x:{}, y:{} could not be mapped to screen", x, y)
                }
            }
        }
    }
    let byte_array = cnv.to_png();
    write("output.png", byte_array).expect("Could not write output.png to disk");
}

#[derive(Debug)]
enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds { x: F, y: F },
}
impl Pixel {
    pub fn from_point_to_canvas(point: VTuple, canvas: &VCanvas) -> Pixel
    {
        if !point.is_point() {
            panic!("Must input a point")
        }
        let x = point.x;
        let y = point.y;

        if x < 0.0 || y < 0.0 {
            return Pixel::OutOfBounds { x: x, y: y };
        }
        if y > canvas.height as f64 {
            return Pixel::OutOfBounds { x: x, y: y };
        }
        let screen_x = x.round() as usize;
        let screen_y = canvas.height - y.round() as usize;

        if (screen_x >= canvas.width) || (screen_y >= canvas.height) {
            return Pixel::OutOfBounds { x: x, y: y };
        } else {
            return Pixel::Coordinate {
                x: screen_x,
                y: screen_y,
            };
        }
    }
}
