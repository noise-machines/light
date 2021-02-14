mod light;
mod prelude;

use nannou::prelude::*;

fn main() {
    nannou::app(start).run();
}

struct Model {}

fn start(app: &App) -> Model {
    prelude::save_current_version_of_source_code();
    app.new_window().view(view).build().unwrap();
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let rand = prelude::get_rand();
    light::draw(app, frame, rand);
}
