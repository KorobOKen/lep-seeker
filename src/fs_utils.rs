use std::path::{Path, PathBuf};
use std::fs;
use std::io::Result;
use std::error::Error;

pub fn get_sources_paths() -> Option<Vec<PathBuf>> {
  let sources_path = Path::new(".")
    .join("pics")
    .join("sources");

  let rd = fs::read_dir(&sources_path);

  if let Err(why) = rd {
    println!("Ошибка при чтении папки '{:?}': {}", sources_path.into_os_string(), why);
    return None;
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
    .map(|dir_entry| dir_entry.unwrap().path())
    .collect::<Vec<_>>();

  Some(paths_vector)
}
