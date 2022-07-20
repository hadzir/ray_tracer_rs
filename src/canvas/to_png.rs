use crate::canvas::to_rgba32::ToRGBA32;
use crate::canvas::vcanvas::Sized;
use png::{Encoder};
use png::ColorType::Rgba;
use png::BitDepth::Eight;
pub trait ToPNG {
  fn to_png(&self) -> Vec<u8>;
}

impl<T> ToPNG for T
where
  T: ToRGBA32,
  T: Sized,
{
  fn to_png(&self) -> Vec<u8> {
    let mut data = Vec::new();
    let mut encoder = Encoder::new(&mut data, self.width() as u32, self.height() as u32);
    encoder.set_color(Rgba);
    encoder.set_depth(Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&self.to_rgba32()).unwrap();
    drop(writer);

    data
  }
}