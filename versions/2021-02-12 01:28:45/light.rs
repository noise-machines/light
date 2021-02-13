use nannou::prelude::*;
use nanorand::RNG;

pub struct Model {}

pub fn start(_app: &App) -> Model {
    Model {}
}

pub fn draw(app: &App, _model: &Model, frame: Frame, mut rand: nanorand::WyRand) {
    let draw = app.draw();

    let x = rand.generate::<u64>() as f32;
    let y = 0.0;

    draw.background().color(PLUM);
    draw.ellipse().x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
