mod light;
mod prelude;
use nannou::prelude::*;

fn main() {
    nannou::app(start).update(event).view(view).run();
}

fn start(app: &App) -> light::Model {
    println!("Starting the sketch.");
    prelude::save_current_version_of_source_code();

    light::start(app)
}

fn event(_app: &App, _model: &mut light::Model, _event: Update) {}

fn view(app: &App, _model: &light::Model, frame: Frame) {
    // let rand = prelude::get_rand();
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
    // light::draw(app, model, frame, rand);
}
