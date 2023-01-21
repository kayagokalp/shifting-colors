use nannou::{
    lyon::geom::{CubicBezierSegment, Point},
    noise::{BasicMulti, NoiseFn},
    prelude::Point2,
};

#[derive(Debug, Clone)]
pub struct BezierCurve {
    bezier_segment: CubicBezierSegment<f32>,
}

impl BezierCurve {
    #[allow(clippy::too_many_arguments)]
    /// Instantiate a new `BezierCurve`.
    pub(crate) fn new(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
    ) -> Self {
        let from = Point::new(x1, y1);
        let ctrl1 = Point::new(x2, y2);
        let ctrl2 = Point::new(x3, y3);
        let to = Point::new(x4, y4);
        let bezier_segment = CubicBezierSegment {
            from,
            ctrl1,
            ctrl2,
            to,
        };
        Self { bezier_segment }
    }

    pub(crate) fn sample(&self, t: f32) -> Point2 {
        let pt = self.bezier_segment.sample(t);
        Point2::new(pt.x, pt.y)
    }

    pub(crate) fn samples(&self) -> Vec<Point2> {
        (0..100)
            .map(|t| t as f32 * 0.01)
            .map(|t| self.sample(t))
            .collect()
    }
    /// Creates a bezier curve from Perlin noise and provided initial state.
    pub(crate) fn from_noise(
        noise: &BasicMulti,
        initial_state: f32,
        limit_x: f32,
        limit_y: f32,
    ) -> Self {
        let sample_state = initial_state + 15.0;
        let x1 = sample_and_limit(noise, limit_x, sample_state);

        let sample_state = initial_state + 55.0;
        let y1 = sample_and_limit(noise, limit_y, sample_state);

        let sample_state = initial_state + 25.0;
        let x2 = sample_and_limit(noise, limit_x, sample_state);

        let sample_state = initial_state + 65.0;
        let y2 = sample_and_limit(noise, limit_y, sample_state);

        let sample_state = initial_state + 35.0;
        let x3 = sample_and_limit(noise, limit_x, sample_state);

        let sample_state = initial_state + 75.0;
        let y3 = sample_and_limit(noise, limit_y, sample_state);

        let sample_state = initial_state + 45.0;
        let x4 = sample_and_limit(noise, limit_x, sample_state);

        let sample_state = initial_state + 5.0;
        let y4 = sample_and_limit(noise, limit_y, sample_state);

        Self::new(x1, y1, x2, y2, x3, y3, x4, y4)
    }
}

fn sample_and_limit(noise: &BasicMulti, limit: f32, offset: f32) -> f32 {
    let sample = noise.get([offset as f64, 0.0]) as f32;
    sample * limit
}
