#[macro_use]
mod debug;
pub mod rand;
mod seed;

use rand::Rand;
use seed::Seed;
use std::path::Path;

fn copy_contents(from: &Path, to: &Path) {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.content_only = true;

    fs_extra::dir::create_all(&to, false).unwrap();
    fs_extra::dir::copy(&from, to, &options).unwrap();
}

fn save_current_version_of_source_code() -> String {
    let manifest_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_folder = manifest_folder.join("src");

    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let target_folder = manifest_folder.join("versions").join(&current_time);

    copy_contents(&source_folder, &target_folder);

    current_time
}

pub fn save() -> Rand {
    let seed = Seed::load();
    dbg!(seed.value);

    seed.save_to_file();

    let checkpoint_name = save_current_version_of_source_code();
    dbg!(checkpoint_name);

    seed.clean_up_file();

    Rand::from_seed(seed.value)
}
