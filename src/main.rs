mod fs_utils;

use fs_utils::get_sources_paths;

fn main() {
  if let Some(paths) = get_sources_paths() {
    println!("{:#?}", paths);
  }
}
