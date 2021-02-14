use crate::helpers::Helpers;
use crate::snapshot::rand::Rand;
use nannou::prelude::*;

pub fn draw(app: &App, frame: &Frame, rand: &mut Rand, helpers: &Helpers) {
    let draw = app.draw();

    let (width, _height) = app.main_window().inner_size_points();
    let x_radius = width / 2.0;

    let random_number = rand.generate();
    dbg!(random_number);

    let x = helpers.w(random_number) - x_radius;
    let y = helpers.h(0.0);

    draw.background().color(PLUM);
    draw.ellipse().x_y(x, y).color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
