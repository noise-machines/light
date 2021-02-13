use nannou::prelude::*;
use nanorand::RNG;

pub fn draw(app: &App, frame: Frame, mut rand: nanorand::WyRand) {
    let draw = app.draw();

    let x = rand.generate_range::<u16>(0, 100) as f32;
    let y = 0.0;
    draw.background().color(PLUM);
    draw.ellipse().x_y(x, y).color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
