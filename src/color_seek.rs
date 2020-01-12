extern crate palette;

use image::{RgbImage, ImageBuffer, Rgb};
use palette::{Hsv, Srgb};
use self::palette::RgbHue;

pub fn get_color_map(src_pic: &RgbImage) -> RgbImage {
  let width = src_pic.width();
  let height = src_pic.height();
  let white_pixels = vec![255u8; (width * height * 3) as usize];
  let mut canvas =
    ImageBuffer::from_vec(width, height, white_pixels)
      .unwrap() as RgbImage;

  for (x, y, src_rgb) in src_pic.enumerate_pixels() {
    let mut hsv = rgb_pixel_to_hsv(&src_rgb);
    hsv.saturation = 1f32;
    hsv.value = 1f32;

    let result_srgb: Srgb = hsv.into();

    canvas.put_pixel(x, y, srgb_to_pixel(result_srgb));
  }

  canvas
}

pub fn get_heat_map(src_pic: &RgbImage) -> RgbImage {
  let width = src_pic.width();
  let height = src_pic.height();
  let white_pixels = vec![255u8; (width * height * 3) as usize];
  let mut canvas =
    ImageBuffer::from_vec(width, height, white_pixels)
      .unwrap() as RgbImage;

  for (x, y, src_rgb) in src_pic.enumerate_pixels() {
    if x < 3 || y < 3 || x > width - 4 || y > height - 4 { continue; }
    let score = count_hue_score(src_pic, x, y);
    let cur_degree: f32 = rgb_pixel_to_hsv(src_rgb).hue.to_positive_degrees();
    let target_hue = RgbHue::from_degrees((cur_degree - score).abs() * 0.7);
    let hsv_color = Hsv::new(target_hue, 1f32, 1f32);
    let result_srgb: Srgb = hsv_color.into();
    canvas.put_pixel(x, y, srgb_to_pixel(result_srgb));
  }

  canvas
}

fn rgb_pixel_to_hsv(color: &Rgb<u8>) -> Hsv {
  let [r, g, b] = color.0;
  Srgb::new(
    r as f32 / 255f32,
    g as f32 / 255f32,
    b as f32 / 255f32,
  ).into()
}

fn count_hue_score(src_pic: &RgbImage, x: u32, y: u32) -> f32 {
  let hue_degrees_sum: f32 = get_surroundings_coordinates(x, y)
    .into_iter()
    .map(|(x, y)|
      rgb_pixel_to_hsv(src_pic.get_pixel(x, y))
        .hue
        .to_positive_degrees()
    )
    .sum();
  // TODO Среднее арифметическое ищется неправильно
  ((hue_degrees_sum / 16f32) % 360f32).round() as f32
}

fn get_surroundings_coordinates(x: u32, y: u32) -> Vec<(u32, u32)> {
  vec![
    (x - 3, y - 0),
    (x - 3, y + 1),
    (x - 2, y + 2),
    (x - 1, y + 3),
    (x - 0, y + 3),
    (x + 1, y + 3),
    (x + 2, y + 2),
    (x + 3, y + 1),
    (x + 3, y + 0),
    (x + 3, y - 1),
    (x + 2, y - 2),
    (x + 1, y - 3),
    (x + 0, y - 3),
    (x - 1, y - 3),
    (x - 2, y - 2),
    (x - 3, y - 1)
  ]
}

fn srgb_to_pixel(src: Srgb) -> Rgb<u8> {
  let (r, g, b) = src.into_components();
  Rgb([
    (r * 255f32).round() as u8,
    (g * 255f32).round() as u8,
    (b * 255f32).round() as u8
  ])
}
