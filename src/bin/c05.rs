use indicatif::ProgressBar;
use ray_tracer::body::VIntersectable;
use ray_tracer::canvas::to_png::ToPNG;
use ray_tracer::canvas::vcanvas::*;
use ray_tracer::canvas::vcolor::VColor;
use ray_tracer::ray::*;
use ray_tracer::sphere::*;
use ray_tracer::tuple::*;
use ray_tracer::F;
use std::fs::write;

fn main() {
    let ray_origin = VTuple::point(0.0, 0.0, -5.0);
    let wall_position_z = 5.0;
    let wall_size = 10.0;

    let canvas_size = 1024;
    let canvas_pixel_world_size = wall_size / canvas_size as F;
    let mut canvas = VCanvas::new(canvas_size, canvas_size);
    let yellow = VColor::yellow();
    let sphere = VSphere::new(None);

    println!(
        "Raytracing {} pixels. Please be patient...",
        canvas_size.pow(2)
    );
    let progress = ProgressBar::new(canvas_size.pow(2) as u64);
    progress.set_draw_rate(2);
    for x in 0..canvas_size {
        for y in 0..canvas_size {
            let half = wall_size / 2.0;
            let world_x = -half + canvas_pixel_world_size * x as f64;
            let world_y = half - canvas_pixel_world_size * y as f64;
            let wall_point = VTuple::point(world_x, world_y, wall_position_z);
            let ray = VRay::new(ray_origin, (wall_point - ray_origin).normalize());
            let xs = sphere.intersect(ray);

            if xs.hit() != None {
                canvas.write_pixel(x, y, yellow);
            }
            progress.inc(1);
        }
    }
    progress.finish();
    let byte_array = canvas.to_png();
    write("output.png", byte_array).expect("Could not write output.png to disk");
}
