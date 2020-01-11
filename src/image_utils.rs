extern crate image;
extern crate imageproc;
extern crate itertools;

use image::{GrayImage, Luma};
use std::path::Path;
use imageproc::corners::Corner;
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

pub fn get_points_map(width: u32, height: u32, corners: Vec<Corner>) -> GrayImage {
  let min_max_score = corners
    .iter()
    .map(|corner| corner.score)
    .minmax_by(|a, b|
      a.partial_cmp(b).unwrap_or(Ordering::Equal)
    );

  let white_pixels = vec![255u8; (width * height) as usize];
  let mut map = GrayImage::from_raw(width, height, white_pixels).unwrap();

  if let MinMax(min, max) = min_max_score {
    let multiplier = 255f32 / (max - min);
    for corner in corners.into_iter() {
      let pixel_value = 255 - ((corner.score as f32 - min) * multiplier) as u8;
      map.put_pixel(corner.x, corner.y, Luma([pixel_value]));
    }
  }

  map
}
