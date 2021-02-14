pub mod rand;
mod seed;
mod source_code;

use rand::Rand;
use seed::Seed;

pub struct Checkpoint {
    pub rand: Rand,
    pub name: String,
    pub seed: u64,
}

pub fn save() -> Checkpoint {
    let seed = Seed::load();
    dbg!(seed.value);

    seed.save_to_file();

    let checkpoint_name = source_code::save_current_version();
    dbg!(&checkpoint_name);

    seed.clean_up_file();

    Checkpoint {
        name: checkpoint_name,
        seed: seed.value,
        rand: Rand::from_seed(seed.value),
    }
}
