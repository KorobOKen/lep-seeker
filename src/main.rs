mod fs_utils;

use fs_utils::get_sources_paths;

fn main() {
  println!("{:#?}", get_sources_paths());
}
