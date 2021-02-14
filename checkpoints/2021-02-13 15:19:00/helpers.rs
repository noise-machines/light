use nannou::prelude::*;
pub struct Helpers<'a> {
    app: &'a App,
}

impl<'a> Helpers<'a> {
    pub fn new(app: &App) -> Helpers {
        Helpers { app }
    }

    pub fn w(&self, x: f32) -> f32 {
        let (width, _height) = self.app.main_window().inner_size_points();
        x * width
    }

    pub fn h(&self, y: f32) -> f32 {
        let (_width, height) = self.app.main_window().inner_size_points();
        y * height
    }
}
