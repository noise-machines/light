// mod light;
mod prelude;
// use nannou::prelude::*;

// fn main() {
//     nannou::app(start).update(event).view(view).run();
// }

// fn start(_app: &App) -> light::Model {
//     println!("Starting the sketch.");

//     // light::start(app)
//     light::Model {}
// }

// fn event(_app: &App, _model: &mut light::Model, _event: Update) {}

// fn view(app: &App, _model: &light::Model, frame: Frame) {
//     // let rand = prelude::get_rand();
//     let draw = app.draw();

//     // set background to blue
//     draw.background().color(BLUE);

//     // put everything on the frame
//     draw.to_frame(app, &frame).unwrap();
//     // light::draw(app, model, frame, rand);
// }

use nannou::prelude::*;

fn main() {
    nannou::app(model).simple_window(view).run();
}

struct Model {
    // _window: window::Id,
}

fn model(_app: &App) -> Model {
    prelude::save_current_version_of_source_code();

    // let _window = app.new_window().view(view).build().unwrap();
    Model {}
}

// fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}
