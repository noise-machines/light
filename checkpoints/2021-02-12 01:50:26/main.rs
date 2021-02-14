mod light;
mod prelude;

use nannou::prelude::*;

fn main() {
    nannou::app(start).simple_window(view).run();
}

struct Model {}

fn start(_app: &App) -> Model {
    prelude::save_current_version_of_source_code();
    // app.set_loop_mode(LoopMode::loop_once());
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let rand = prelude::get_rand();
    light::draw(app, frame, rand);
}
