use core::panic;
use std::fs::write;

use ray_tracer::canvas::vcanvas::*;
use ray_tracer::canvas::vcolor::*;
use ray_tracer::tuple::*;
use ray_tracer::canvas::to_png::*;
use ray_tracer::canvas::to_ppm::*;

fn main() {
    //Time
    let mut t = 0.0;
    let dt = 0.01;
    //Projectile and world
    let environment = Environment::new(
        VTuple::vector(0.0, -0.1, 0.0),
        VTuple::vector(-0.02, 0.0, 0.0),
    );
    let mut projectile = Projectile::new(
        VTuple::point(0.0, 1.0, 0.0),
        VTuple::vector(1.0, 1.8, 0.0).normalized() * 11.25,
    );
    //Canvas space
    let mut cnv = VCanvas::new(1000, 1000);
    while projectile.position.y > 0.0 {
        t += dt;

        let pixel = Pixel::from_point_to_canvas(projectile.position, &cnv);
        match pixel {
            Pixel::Coordinate { x, y } => {
                cnv.write_pixel(x, y, VColor::red());
            }
            _ => {}
        }

        projectile = tick(projectile, &environment, dt);
    }
    println!("Finito => {:.2}", t);

    let byte_array = cnv.to_ppm();
    write("output.ppm", byte_array).expect("Could not write output.ppm to disk");
    let byte_array = cnv.to_png();
    write("output.png", byte_array).expect("Could not write output.png to disk");
}
#[derive(Debug)]
struct Environment {
    gravity: VTuple,
    wind: VTuple,
}
impl Environment {
    pub fn new(gravity: VTuple, wind: VTuple) -> Self {
        Environment { gravity, wind }
    }
}
#[derive(Debug)]
struct Projectile {
    position: VTuple,
    velocity: VTuple,
}
impl Projectile {
    pub fn new(position: VTuple, velocity: VTuple) -> Self {
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
    pub fn from_point_to_canvas(point: VTuple, canvas: &VCanvas) -> Pixel {
        if !point.is_point() {
            panic!("Must input a point")
        }

        let screen_x = point.x.round() as usize;
        let screen_y = canvas.height - point.y.round() as usize;

        if (screen_x >= canvas.width) || (screen_y >= canvas.height) {
            return Pixel::OutOfBounds;
        } else {
            return Pixel::Coordinate {
                x: screen_x,
                y: screen_y,
            };
        }
    }
}
