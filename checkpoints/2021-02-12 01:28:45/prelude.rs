use std::path::Path;

fn copy_contents(from: &Path, to: &Path) {
    let mut options = fs_extra::dir::CopyOptions::new();
    options.content_only = true;

    fs_extra::dir::create_all(&to, false).unwrap();
    fs_extra::dir::copy(&from, to, &options).unwrap();
}

pub fn save_current_version_of_source_code() {
    let manifest_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    let source_folder = manifest_folder.join("src");

    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let target_folder = manifest_folder.join("versions").join(current_time);

    copy_contents(&source_folder, &target_folder);
}

#[allow(dead_code)]
pub fn get_rand_with_constant_seed() -> nanorand::WyRand {
    let seed = 36251;
    nanorand::WyRand::new_seed(seed as u64)
}

#[allow(dead_code)]
pub fn get_rand() -> nanorand::WyRand {
    let seed = chrono::Local::now().timestamp_nanos();
    nanorand::WyRand::new_seed(seed as u64)
}
