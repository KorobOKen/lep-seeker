use std::path::Path;
use std::fs;
use std::io::Result;
use std::ffi::OsString;

pub fn get_sources_paths() -> Vec<OsString> {
  let sources_path = Path::new(".")
    .join("pics")
    .join("sources");

  let (paths, errors) = fs::read_dir(sources_path)
    .unwrap()
    .partition::<Vec<_>, _>(Result::is_ok);

  for err in errors {
    if let Err(err) = err {
      println!("Ошибка: {}", err);
    }
  }

  paths
    .into_iter()
    .map(|dir_entry| dir_entry
      .unwrap()
      .path()
      .into_os_string()
    )
    .collect::<Vec<_>>()
}
