mod helpers;
mod light;
mod snapshot;
use helpers::Helpers;

use nannou::prelude::*;

fn main() {
    nannou::app(start)
        .simple_window(view)
        .exit(snapshot::exit)
        .run();
}

pub struct Model {}

fn start(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_ntimes(3));

    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let mut current_snapshot = snapshot::save(app);
    let helpers = Helpers::new(app);

    light::draw(app, &frame, &mut current_snapshot.rand, &helpers);

    current_snapshot.clean_up(app);
}
