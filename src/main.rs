mod utils;

use nannou::prelude::*;
use utils::bezier::BezierCurve;

struct Model {
    initial_state: f32,
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model { initial_state: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    let t = app.time;

    let bezier_curve = BezierCurve::new(100.0, 100.0, 120.0, 50.0, 50.0, 200.0, 220.0, 100.0);
    let samples = bezier_curve.samples();
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

    // Clear the background to purple.
    draw.background().color(WHITESMOKE);
    draw.to_frame(app, &frame).unwrap();
}
