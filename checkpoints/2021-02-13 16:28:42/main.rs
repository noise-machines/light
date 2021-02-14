mod light;
#[macro_use]
mod checkpoint;
mod helpers;
use helpers::Helpers;

use nannou::prelude::*;

fn main() {
    nannou::app(start).simple_window(view).run();
}

struct Model {}

fn start(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());

    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let mut current_checkpoint = checkpoint::save();
    let helpers = Helpers::new(app);

    for _ in 1..=10 {
        app.main_window()
            .capture_frame(&current_checkpoint.image_path);

        light::draw(app, &frame, &mut current_checkpoint.rand, &helpers);
    }
}
