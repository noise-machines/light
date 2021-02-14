use crate::checkpoint::rand::Rand;
use nannou::prelude::*;

pub fn draw(app: &App, frame: Frame, mut rand: Rand) {
    let draw = app.draw();

    let dimensions = app.main_window().inner_size_points();
    let width = dimensions.0 as f32;
    let height = dimensions.1 as f32;
    let x_radius = width / 2.0;

    dbg!(width);

    let w = |x: f32| x * width;
    let h = |y: f32| y * height;

    let random_number = rand.generate();
    // let random_number = 1.0;
    dbg!(random_number);

    let x = w(random_number) - x_radius;
    let y = h(0.0);
    dbg!(x);

    draw.background().color(PLUM);
    draw.ellipse().x_y(x, y).color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
