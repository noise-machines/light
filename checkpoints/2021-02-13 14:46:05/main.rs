mod light;
#[macro_use]
mod checkpoint;

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
    let rand = checkpoint::save();

    light::draw(app, frame, rand);
    app.main_window()
        .capture_frame(app.exe_name().unwrap() + ".png");
}
