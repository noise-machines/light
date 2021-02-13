#[macro_use]
mod debug;
mod seed;

use seed::Seed;
use std::path::Path;

fn copy_contents(from: &Path, to: &Path) {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.content_only = true;

    fs_extra::dir::create_all(&to, false).unwrap();
    fs_extra::dir::copy(&from, to, &options).unwrap();
}

fn save_current_version_of_source_code() {
    let manifest_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_folder = manifest_folder.join("src");

    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let target_folder = manifest_folder.join("versions").join(current_time);

    copy_contents(&source_folder, &target_folder);
}

pub fn save() -> nanorand::WyRand {
    let seed = Seed::load();
    debug!(seed.value);

    seed.save_to_file();

    save_current_version_of_source_code();

    seed.clean_up_file();

    nanorand::WyRand::new_seed(seed.value)
}
