mod checkpoint;
mod helpers;
mod light;
use helpers::Helpers;

use nannou::prelude::*;

fn main() {
    nannou::app(start)
        .simple_window(view)
        .exit(checkpoint::exit)
        .run();
}

pub struct Model {}

fn start(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_ntimes(3));

    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let mut current_checkpoint = checkpoint::save(app);
    let helpers = Helpers::new(app);

    light::draw(app, &frame, &mut current_checkpoint.rand, &helpers);

    current_checkpoint.clean_up(app);
}
