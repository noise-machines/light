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
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    prelude::save_current_version_of_source_code();
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
