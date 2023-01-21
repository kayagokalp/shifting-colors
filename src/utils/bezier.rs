use nannou::{
    lyon::geom::{CubicBezierSegment, Point},
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
}
