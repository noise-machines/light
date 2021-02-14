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
}

impl Checkpoint {
    pub fn image_path(&self) -> PathBuf {
        manifest::folder().join("images").join(self.image_name())
    }

    pub fn symlink_image_into_checkpoints_directory(&self) {
        let image_path = self.image_path();
        let symlink_path = self.image_symlink_path();
        std::os::unix::fs::symlink(image_path, symlink_path).unwrap();
    }

    fn image_symlink_path(&self) -> PathBuf {
        manifest::folder()
            .join("checkpoints")
            .join(&self.name)
            .join(self.image_name())
    }

    fn image_name(&self) -> String {
        format!("{}{}", &self.name, ".png")
    }
}

fn get_checkpoint_name(frame_number: u64) -> String {
    let checkpoint_name: String = frame_number.to_string();
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    format!("{} {}", current_time, checkpoint_name)
}

pub fn save(frame_number: u64) -> Checkpoint {
    let checkpoint_name = get_checkpoint_name(frame_number);

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
    }
}
