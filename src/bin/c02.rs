use core::panic;
use std::fs::write;

use ray_tracer::canvas::*;
use ray_tracer::color::*;
use ray_tracer::tuple::*;

fn main() {
    //Time
    let mut t = 0.0;
    let dt = 0.1;
    //Projectile and world
    let environment = Environment::new(
        VTuple::vector(0.0, -0.1, 0.0),
        VTuple::vector(-0.02, 0.0, 0.0),
    );
    let mut projectile = Projectile::new(
        VTuple::point(0.0, 1.0, 0.0),
        VTuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    );
    //Canvas space
    let mut cnv = VCanvas::new(1000, 1000);
    while projectile.position.y > 0.0 {
        t += dt;
        
        let pixel = Pixel::from_point_to_canvas(projectile.position, &cnv);
        match pixel{
            Pixel::Coordinate{x,y}=>{cnv.write_pixel(x, y, VColor::red());}
            _=>{}
        }
        

        projectile = tick(projectile, &environment, dt);
    }
    println!("Finito => {:.2}", t);

    let byte_array = cnv.to_ppm();
    write("output.ppm", byte_array).expect("Could not write output.ppm to disk")
}
#[derive(Debug)]
struct Environment {
    gravity: VTuple<f64>,
    wind: VTuple<f64>,
}
impl Environment {
    pub fn new(gravity: VTuple<f64>, wind: VTuple<f64>) -> Self {
        Environment { gravity, wind }
    }
}
#[derive(Debug)]
struct Projectile {
    position: VTuple<f64>,
    velocity: VTuple<f64>,
}
impl Projectile {
    pub fn new(position: VTuple<f64>, velocity: VTuple<f64>) -> Self {
        Projectile { position, velocity }
    }
}

fn tick(projectile: Projectile, environment: &Environment, dt: f64) -> Projectile {
    Projectile::new(
        projectile.position + projectile.velocity * dt,
        projectile.velocity + environment.gravity * dt + environment.wind * dt,
    )
}
enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds,
}
impl Pixel {
    pub fn from_point_to_canvas(point: VTuple<f64>, canvas: &VCanvas) -> Pixel {
        if !point.is_point() {
            panic!("Must input a point")
        }

        let screen_x = point.x.round() as usize;
        let screen_y = canvas.height - point.y.round() as usize;

        if (screen_x >= canvas.width) || (screen_y >= canvas.height)
        {
            return Pixel::OutOfBounds;
        } else {
            return Pixel::Coordinate {
                x: screen_x,
                y: screen_y,
            };
        }
    }
}
