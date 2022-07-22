use indicatif::ProgressBar;
use itertools::Itertools;
use ray_tracer::animator::*;
use ray_tracer::body::VBody;
use ray_tracer::camera::VCamera;
use ray_tracer::canvas::to_png::ToPNG;
use ray_tracer::canvas::vcanvas::*;
use ray_tracer::canvas::vcolor::VColor;
use ray_tracer::light::VPointLight;
use ray_tracer::material::VMaterial;
use ray_tracer::material::VPhong;
use ray_tracer::matrix::VMatrix;
use ray_tracer::sphere::*;
use ray_tracer::tuple::*;
use ray_tracer::world::VWorld;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::fs::write;
use std::sync::Mutex;

fn main() {
    let animator = VAnimator::new(10);
    animator.animate(|frame| {
        //World params
        let canvas_width = 600;
        let canvas_height = 400;

        let light_rotation = frame.linear_scale().with_range(vec![0.0, PI * 2.0]);
        let light_transform = VMatrix::rotation_y(light_rotation.scale(frame.currentf()));
        let light = VPointLight::new(
            light_transform * VTuple::point(-5.0, 15.0, -10.0),
            VColor::new(0.9, 0.9, 0.9),
        );

        println!("{:?}",light_transform* VTuple::point(-5.0, 15.0, -10.0));

        let pixel_count = canvas_width * canvas_height;
        let canvas_mutex = Mutex::new(VCanvas::new(canvas_width, canvas_height));
        let camera = VCamera::new(canvas_width, canvas_height, PI / 3.0).positioned_and_pointed(
            VTuple::point(-10.0, 10.0, -10.0),
            VTuple::point(0.0, 0.0, 0.0),
            VTuple::vector(0.0, 1.0, 0.0),
        );

        //World objects
        let material1 = VMaterial::from(VPhong {
            col: VColor::red(),
            ..VPhong::default()
        });
        let sphere1 = VSphere::default()
            .with_material(material1)
            .with_transform(VMatrix::translation(-3.0, 1.0, -2.0));

        let material2 = VMaterial::from(VPhong {
            col: VColor::green(),
            ..VPhong::default()
        });
        let sphere2 = VSphere::default()
            .with_material(material2)
            .with_transform(VMatrix::translation(-3.0, 1.0, -6.0));

        let wall_mat = VMaterial::from(VPhong {
            col: VColor::new(0.2, 0.2, 0.2),
            spc: 0.0,
            ..VPhong::default()
        });
        let floor = VSphere::default()
            .with_material(wall_mat)
            .with_transform(VMatrix::scaling(10.0, 0.01, 10.0));
        let wall1 = VSphere::default()
            .with_material(wall_mat)
            .with_transform(VMatrix::scaling(0.01, 10.0, 10.0));
        let wall2 = VSphere::default()
            .with_material(wall_mat)
            .with_transform(VMatrix::scaling(10.0, 10.0, 0.01));

        let world = VWorld::new(
            vec![
                VBody::from(sphere1),
                VBody::from(sphere2),
                VBody::from(floor),
                VBody::from(wall1),
                VBody::from(wall2),
            ],
            vec![light],
        );

        println!("Raytracing {} pixels. Please be patient...", pixel_count);
        let progress = ProgressBar::new(pixel_count as u64);
        progress.set_draw_rate(2);
        (0..canvas_width) //x
            .cartesian_product(0..canvas_height) //y
            .par_bridge()
            .for_each(|(x, y)| {
                let ray = camera.ray_for_pixel(x, y);

                let col = world.color_at(ray);
                let mut canvas = canvas_mutex.lock().unwrap();
                canvas.write_pixel(x, y, col);
                progress.inc(1);
            });
        progress.finish();

        let filename = frame.filename("animation", "output", ".png");
        println!("Writing {filename:?}");
        let canvas = canvas_mutex.lock().unwrap();
        let byte_array = canvas.to_png();
        write(filename, byte_array).expect("Couldnt write {filename:?} to disk");
    });
}
