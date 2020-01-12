mod fs_utils;
mod image_utils;
mod color_seek;

use fs_utils::get_sources_paths;
use image_utils::{get_luma_by_path, get_points_map, get_rgb_by_path};
use imageproc::corners::corners_fast9;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use color_seek::get_color_map;

fn main() {
  let paths = get_sources_paths();
  if let None = paths { return; }
  for path in paths.unwrap() {
    let pic_name = path.file_stem().unwrap();
    println!("{:?}", pic_name);

    if let Some(luma) = get_luma_by_path(&path) {
      println!("luma for {} calculated", pic_name.to_string_lossy());
      let corners9 = corners_fast9(&luma, 50);
      println!("Corners 9 amount: {}", corners9.len());
      let map = get_points_map(
        luma.width(),
        luma.height(),
        corners9,
      );
      println!("Points map built for {}", pic_name.to_string_lossy());
      map.save(get_result_full_path(pic_name, "corners")).unwrap();
    }

    if let Some(rgb) = get_rgb_by_path(&path) {
      println!("rgb for {} calculated", pic_name.to_string_lossy());
      let map = get_color_map(&rgb);
      println!("color map for {} calculated", pic_name.to_string_lossy());
      map.save(get_result_full_path(pic_name, "hsv")).unwrap();
    }

    println!();
  }
}

fn get_result_full_path(pic_name: &OsStr, method: &str) -> PathBuf {
  let mut file_name = pic_name.to_os_string();
  file_name.push(format!(".{}", method));
  file_name.push(".png");
  Path::new(".")
    .join("pics")
    .join(file_name)
}
