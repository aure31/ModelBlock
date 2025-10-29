use crate::utils::VectorPoint;
use std::sync::Arc;

pub trait VectorInterpolation {
    fn interpolate(&self, points: &[VectorPoint], p2_index: usize, time: f32) -> VectorPoint;

    fn is_continuous(&self) -> bool {
        true
    }
}

pub fn default_interpolation() -> Arc<LinearInterpolation> {
    Arc::new(LinearInterpolation)
}

pub struct LinearInterpolation;

impl VectorInterpolation for LinearInterpolation {
    fn interpolate(&self, points: &[VectorPoint], p2_index: usize, time: f32) -> VectorPoint {
        let p1 = if p2_index > 0 {
            &points[p2_index - 1]
        } else {
            &VectorPoint::empty()
        };
        let p2 = &points[p2_index];
        let t = (time - p1.time) / (p2.time - p1.time);
        VectorPoint::new(
            p1.vector.lerp(&p2.vector, t),
            time,
            Arc::new(LinearInterpolation),
        )
    }
}
