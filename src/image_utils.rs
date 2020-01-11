extern crate image;
extern crate imageproc;

use self::image::GrayImage;
use std::path::Path;

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
