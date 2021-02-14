use crate::checkpoint::rand::Rand;
use nannou::prelude::*;

pub fn draw(app: &App, frame: Frame, mut rand: Rand) {
    let draw = app.draw();
    let (width, height) = app.main_window().inner_size_pixels();

    let w = |x: f32| x * (width as f32);
    let h = |y: f32| y * (height as f32);

    let random_number = rand.generate();
    dbg!(random_number);

    let x = w(random_number);
    let y = h(0.0);
    dbg!(x);

    draw.background().color(PLUM);
    draw.ellipse().x_y(x, y).color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
