mod utils;

use nannou::{noise::BasicMulti, prelude::*};
use utils::bezier::BezierCurve;

struct Model {
    noise: BasicMulti,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).fullscreen().run();
}

fn model(_app: &App) -> Model {
    Model {
        noise: BasicMulti::new(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    // Dilluting t down so that draw calls are sufficiently spaced out.
    let t = app.time * 0.1;

    let (width, height) = app.main_window().inner_size_pixels();

    let noise = &model.noise;

    // Instantiate a bezier curve.
    let bezier_curve = BezierCurve::from_noise(noise, t, width as f32, height as f32);
    //let bezier_curve = BezierCurve::new(100.0, 100.0, 120.0, 50.0, 50.0, 200.0, 220.0, 100.0);
    // Get all samples from the curve.
    let samples = bezier_curve.samples();
    // Map samples and obtain colors from each of them.
    let vertices = samples.iter().enumerate().map(|(index, point)| {
        // There will always be 100 points for a curve.
        let fract = index as f32 / 100.0;
        let r = (t + fract) % 1.0;
        let g = (t + 1.0 - fract) % 1.0;
        let b = (t + 0.5 + fract) % 1.0;
        let rgba = srgba(r, g, b, 1.0);
        (*point, rgba)
    });

    // Draw the polyline as a stroked path.
    draw.polyline().points_colored(vertices);

    draw.to_frame(app, &frame).unwrap();
}
