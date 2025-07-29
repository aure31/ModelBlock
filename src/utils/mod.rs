use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;

use ordered_float::OrderedFloat;
use pumpkin_util::math::vector3::Vector3;

use crate::data::blueprint::animation::AnimationPoint;

pub mod math;

#[derive(Clone)]
pub struct VectorPoint {
    pub vector: Vector3<f32>,
    pub time: f32,
    pub interpolation: Arc<dyn VectorInterpolation + Sync + Send>,
}

fn points(set: &mut BTreeSet<OrderedFloat<f32>>, points: &[VectorPoint]) {
    for point in points {
        set.insert(OrderedFloat(point.time));
    }
}

pub fn sum(
    lenght: f32,
    position: &[VectorPoint],
    rotation: &[VectorPoint],
    scale: &[VectorPoint],
) -> Vec<AnimationPoint> {
    let mut set: BTreeSet<OrderedFloat<f32>> = BTreeSet::new();
    set.insert(OrderedFloat(0.));
    set.insert(OrderedFloat(lenght));
    points(&mut set, &position);
    points(&mut set, &rotation);
    points(&mut set, &scale);
    sum_with_set(position, rotation, scale, set)
}

fn sum_with_set(
    position: &[VectorPoint],
    rotation: &[VectorPoint],
    scale: &[VectorPoint],
    set: BTreeSet<OrderedFloat<f32>>,
) -> Vec<AnimationPoint> {
    let mut points = Vec::new();
    let pp = put_point(&position, &set);
    let rp = put_point(&rotation, &set);
    let sp = put_point(&scale, &set);
    for ((position, rotation), scale) in pp.into_iter().zip(rp).zip(sp) {
        points.push(AnimationPoint {
            position,
            rotation,
            scale,
        });
    }
    points
}

fn put_point(vectors: &[VectorPoint], points: &BTreeSet<OrderedFloat<f32>>) -> Vec<VectorPoint> {
    let mut new_points = Vec::with_capacity(points.len());

    if vectors.len() < 2 {
        let first = vectors.get(0).cloned().unwrap_or_else(VectorPoint::empty);
        for &time in points {
            new_points.push(VectorPoint::new(
                first.vector,
                *time,
                first.interpolation.clone(),
            ));
        }
        return new_points;
    }
    let mut p1 = &VectorPoint::empty();
    let mut p2 = &vectors[0];
    let last = &vectors.last().expect("Should have a last point");
    let lenght = last.time;
    let mut i = 0;
    let mut t = p2.time;
    for &time_point in points {
        while i < vectors.len() - 1 && t < *time_point {
            p1 = p2;
            i += 1;
            p2 = &vectors[i];
            t = p2.time;
        }
        if time_point > lenght.into() {
            new_points.push(VectorPoint::new(
                last.vector,
                *time_point,
                last.interpolation.clone(),
            ));
        } else if time_point == t {
            new_points.push(vectors[i].clone());
        } else {
            new_points.push(p1.interpolation.interpolate(vectors, i, *time_point));
        }
    }
    if t < lenght {
        new_points.extend_from_slice(&vectors[i..]);
    }
    new_points
}

impl VectorPoint {
    pub fn new(
        vector: Vector3<f32>,
        time: f32,
        interpolation: Arc<dyn VectorInterpolation + Sync + Send>,
    ) -> Self {
        Self {
            vector,
            time,
            interpolation,
        }
    }

    pub fn empty() -> Self {
        Self {
            vector: Vector3::new(0.0, 0.0, 0.0),
            time: 0.0,
            interpolation: default_interpolation(),
        }
    }
}

pub trait VectorInterpolation {
    fn interpolate(&self, points: &[VectorPoint], p2_index: usize, time: f32) -> VectorPoint;
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
