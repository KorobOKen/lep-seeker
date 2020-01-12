extern crate palette;

use image::{RgbImage, ImageBuffer, Rgb};
use palette::Hsv;
use palette::rgb::Srgb;

pub fn get_color_map(src_pic: &RgbImage) -> RgbImage {
  let width = src_pic.width();
  let height = src_pic.height();

  let white_pixels = vec![255u8; (width * height * 3) as usize];
  let mut canvas =
    ImageBuffer::from_vec(width, height, white_pixels)
      .unwrap() as RgbImage;

  for (x, y, src_rgb) in src_pic.enumerate_pixels() {
    let [r, g, b] = src_rgb.0;
    let mut hsv: Hsv = Srgb::new(
      r as f32 / 255f32,
      g as f32 / 255f32,
      b as f32 / 255f32,
    )
      .into();
    hsv.saturation = 1f32;
    hsv.value = 1f32;
    let result_srgb: Srgb = hsv.into();
    let (r, g, b) = result_srgb.into_components();
    let color = Rgb([
      (r * 255f32).round() as u8,
      (g * 255f32).round() as u8,
      (b * 255f32).round() as u8
    ]);
    canvas.put_pixel(x, y, color);
  }

  canvas
}
