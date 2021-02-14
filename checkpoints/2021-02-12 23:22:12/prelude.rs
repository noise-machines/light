use std::fs;
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

fn save_seed(seed: u64) {
    let manifest_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    let seed_path = manifest_folder.join("src").join("seed");
    fs::write(seed_path, seed.to_string()).expect("Unable to write file");
}

fn delete_seed() {
    let manifest_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    let seed_path = manifest_folder.join("src").join("seed");
    fs::remove_file(seed_path).unwrap();
}

pub fn save_state() -> nanorand::WyRand {
    let seed_path = Path::new(file!()).join("seed");
    let need_to_create_seed_file = !seed_path.exists();
    println!("Need to create seed file? {:?}", need_to_create_seed_file);

    let seed = if need_to_create_seed_file {
        get_seed_from_current_time()
    } else {
        get_seed_from_file(&seed_path)
    };

    if need_to_create_seed_file {
        save_seed(seed);
    }

    save_current_version_of_source_code();

    // The seed file is already saved in the
    // appropriate versions/ subfolder, together
    // with our source code.
    //
    // We delete it here so that we get a new seed
    // next time we compile.
    if need_to_create_seed_file {
        delete_seed();
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
