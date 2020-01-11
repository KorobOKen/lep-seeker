mod fs_utils;
mod image_utils;

use fs_utils::get_sources_paths;
use image_utils::{get_luma_by_path, get_points_map};
use imageproc::corners::corners_fast9;
use std::path::Path;

fn main() {
  let paths = get_sources_paths();
  if let None = paths { return; }
  for os_path in paths.unwrap() {
    println!("{:?}", &os_path);
    if let Some(luma) = get_luma_by_path(&os_path) {
      println!("luma for {:?} calculated", &os_path);
      let corners9 = corners_fast9(&luma, 80);
      println!("Corners 9 amount: {}", corners9.len());
//      let corners12 = corners_fast12(&luma, 100);
//      println!("Corners 12 amount: {}", corners12.len());
      let map = get_points_map(
        luma.width(),
        luma.height(),
        corners9,
      );
      let result_path = Path::new(".")
        .join("pics")
        .join("results")
        .join("result.png");
      map.save(&result_path).unwrap();
      println!("__________________");
    }
  }
}
