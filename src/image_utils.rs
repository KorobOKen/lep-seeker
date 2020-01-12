extern crate image;
extern crate imageproc;
extern crate itertools;

use image::{GrayAlphaImage, GrayImage, LumaA, ImageBuffer};
use std::path::Path;
use imageproc::corners::Corner;
use imageproc::drawing::draw_filled_circle_mut;
use std::cmp::Ordering;
use itertools::Itertools;
use itertools::MinMaxResult::*;

pub fn get_luma_by_path<P: AsRef<Path>>(path: P) -> Option<GrayImage> {
  match image::open(&path) {
    Err(why) => {
      let path_string = path.as_ref().as_os_str().to_string_lossy();
      println!("Ошибка обработки изображения {}: {}", path_string, why);
      None
    }
    Ok(image) => Some(image.to_luma())
  }
}

pub fn get_points_map(width: u32, height: u32, corners: Vec<Corner>) -> GrayAlphaImage {
  let min_max_score = corners
    .iter()
    .map(|corner| corner.score)
    .minmax_by(|a, b|
      a.partial_cmp(b).unwrap_or(Ordering::Equal)
    );

  let white_pixels = vec![255u8; (width * height * 2) as usize];
  let mut map = ImageBuffer::from_raw(width, height, white_pixels).unwrap();

  if let MinMax(min, max) = min_max_score {
    let multiplier = 255f32 / (max - min);
    for corner in corners.into_iter() {
      let pixel_value = 255u8 - ((corner.score as f32 - min) * multiplier).round() as u8;
      let color = LumaA([pixel_value, 255 - pixel_value]);
      let radius = match &pixel_value {
        v if *v < 40 => 5,
        v if *v < 70 => 4,
        v if *v < 110 => 3,
        v if *v < 155 => 2,
        v if *v < 190 => 1,
        _ => {
          map.put_pixel(corner.x, corner.y, color);
          continue;
        }
      };
      draw_filled_circle_mut(&mut map, (corner.x as i32, corner.y as i32), radius, color);
    }
  }

  map
}
