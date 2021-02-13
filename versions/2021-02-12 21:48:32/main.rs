mod light;
mod prelude;

use nannou::prelude::*;
use nanorand::RNG;

fn main() {
    nannou::app(start).simple_window(view).run();
}

struct Model {}

fn start(app: &App) -> Model {
    prelude::save_current_version_of_source_code();
    app.set_loop_mode(LoopMode::loop_once());

    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let mut rand = prelude::get_rand_with_seed(1613184486665184000);

    let random_number = rand.generate_range::<u64>(0, 100);
    println!("{:?}", random_number);

    light::draw(app, frame, rand);
    app.main_window()
        .capture_frame(app.exe_name().unwrap() + ".png");
}
