use std::fs;
use std::path::Path;

#[cfg(debug_assertions)]
macro_rules! debug {
    ($x:expr) => {
        dbg!($x)
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($x:expr) => {
        std::convert::identity($x)
    };
}

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

pub fn save_state() -> nanorand::WyRand {
    let manifest_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    let seed_path = manifest_folder.join("src").join("seed");
    let need_to_create_seed_file = !seed_path.exists();

    debug!(&seed_path);
    debug!(need_to_create_seed_file);

    let seed = if need_to_create_seed_file {
        get_seed_from_current_time()
    } else {
        get_seed_from_file(&seed_path)
    };

    debug!(seed);

    if need_to_create_seed_file {
        fs::write(&seed_path, seed.to_string()).unwrap();
    }

    save_current_version_of_source_code();

    // The seed file is already saved in the
    // appropriate versions/ subfolder, together
    // with our source code.
    //
    // We delete it here so that we get a new seed
    // next time we compile.
    if need_to_create_seed_file {
        fs::remove_file(seed_path).unwrap();
    }

    nanorand::WyRand::new_seed(seed)
}

fn get_seed_from_file(seed_path: &Path) -> u64 {
    let seed = std::fs::read_to_string(seed_path).unwrap();
    seed.parse::<u64>().unwrap()
}

fn get_seed_from_current_time() -> u64 {
    chrono::Local::now().timestamp_nanos() as u64
}