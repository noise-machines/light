mod manifest;
pub mod rand;
mod seed;
mod source_code;

use rand::Rand;
use seed::Seed;
use std::path::PathBuf;

pub struct Checkpoint {
    pub rand: Rand,
    pub name: String,
    pub seed: u64,
    pub image_path: PathBuf,
}

fn get_image_path(checkpoint_name: &str) -> PathBuf {
    let image_name = format!("{}{}", checkpoint_name, ".png");
    manifest::folder().join("images").join(image_name)
}

pub fn save() -> Checkpoint {
    let checkpoint_name = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let image_path = get_image_path(&checkpoint_name);

    let seed = Seed::load();
    seed.save_to_file();
    source_code::save_current_version(&checkpoint_name);
    seed.clean_up_file();

    dbg!(&checkpoint_name);
    dbg!(seed.value);

    Checkpoint {
        name: checkpoint_name,
        seed: seed.value,
        rand: Rand::from_seed(seed.value),
        image_path,
    }
}