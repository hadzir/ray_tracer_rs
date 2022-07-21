use body::*;
use canvas::vcolor::VColor;
use intersections::*;
use light::VPointLight;
use light::*;
use material::*;
use material::{VMaterial, VPhong};
use ray::*;
use sphere::VSphere;
use std::panic;
use tuple::VTuple;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeAlloc = wee_alloc::WeAlloc::INIT;

#[macro_use]
pub mod zequality;
pub mod body;
pub mod canvas;
pub mod intersections;
pub mod light;
pub mod material;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod tuple;
pub type F = f64;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}
#[wasm_bindgen]
pub struct World {
    canvas_size: u32,
    ray_origin: VTuple,
    wall_position_z: F,
    wall_size: F,
    canvas_pixel_world_size: F,
    sphere: VSphere,
    light: VPointLight,
}
#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_size: u32,r:f64,g:f64,b:f64) -> Self {
        let material = VMaterial::from(VPhong::default().with_color(VColor::new(r, g, b)));
        let wall_size = 5.0;

        World {
            canvas_size,
            ray_origin: VTuple::point(0.0, 0.0, -10.0),
            wall_position_z: 9.0,
            wall_size,
            canvas_pixel_world_size: wall_size / canvas_size as f64,
            sphere: VSphere::default().with_material(material),
            light: VPointLight::new(
                VTuple::point(-10.0, 10.0, -10.0),
                VColor::new(0.9, 0.9, 0.9),
            ),
        }
    }
    pub fn render(&self, y: f64) -> Result<ImageData, JsValue> {
        let data_size = self.canvas_size as usize * 4;
        let mut data: Vec<u8> = Vec::with_capacity(data_size);

        unsafe { data.set_len(data_size) }

        (0..self.canvas_size).for_each(|x| {
            let half = self.wall_size / 2.0;
            let world_x = -half + self.canvas_pixel_world_size * x as f64;
            let world_y = half - self.canvas_pixel_world_size * y as f64;
            let wall_point = VTuple::point(world_x, world_y, self.wall_position_z);
            let ray = VRay::new(self.ray_origin, (wall_point - self.ray_origin).normalize());
            let xs = self.sphere.intersect(ray);

            let hit = xs.hit();

            let mut col = VColor::black();
            if let Some(hit) = hit {
                let computed = hit.get_computed();
                col = hit.body.material().lighting(
                    self.light,
                    computed.pos,
                    computed.camv,
                    computed.normalv,
                );
            }
            #[allow(clippy::identity_op)]
            {
                data[(x * 4 + 0) as usize] = (col.r*255.0).round() as u8;
                data[(x * 4 + 1) as usize] = (col.g*255.0).round() as u8;
                data[(x * 4 + 2) as usize] = (col.b*255.0).round() as u8;
                data[(x * 4 + 3) as usize] = 255;
            }
        });
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.canvas_size, 1)
    }
}
