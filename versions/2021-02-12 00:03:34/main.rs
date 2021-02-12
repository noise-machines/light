#![feature(const_in_array_repeat_expressions)]

mod prelude;
use nannou::prelude::*;

fn main() {
    nannou::app(start).update(event).view(view).run();
}

struct Model {}

fn start(_app: &App) -> Model {
    println!("Starting the sketch.");
    prelude::save_current_version_of_source_code();

    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    let x = 0.0;
    let y = 0.0;

    draw.background().color(PLUM);
    draw.ellipse().x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
