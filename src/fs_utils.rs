use std::path::Path;
use std::fs;
use std::io::Result;
use std::ffi::OsString;
use std::error::Error;

pub fn get_sources_paths() -> Option<Vec<OsString>> {
  let sources_path = Path::new(".")
    .join("pics")
    .join("sources");

  let rd = fs::read_dir(&sources_path);

  if let Err(why) = rd {
    println!("Ошибка при чтении папки '{:?}': {}", sources_path.into_os_string(), why);
    return None
  }

  let (paths, errors) = rd.unwrap()
    .partition::<Vec<_>, _>(Result::is_ok);

  for err in errors {
    if let Err(err) = err {
      println!("Ошибка {:?}: {}", err.kind(), err.description());
    }
  }

  let paths_vector = paths
    .into_iter()
    .map(|dir_entry| dir_entry
      .unwrap()
      .path()
      .into_os_string()
    )
    .collect::<Vec<_>>();

  Some(paths_vector)
}
