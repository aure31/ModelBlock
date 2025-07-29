use std::f32::consts::PI;

use pumpkin_util::math::vector3::Vector3;

use crate::data::raw::float3::Float3;

pub const MINECRAFT_TICK_MILIS: u8 = 50;
pub const ROTATION_DEGREE: f32 = 22.5;
pub const DEGREE_TO_RADIANS: f32 = PI / 180.;
pub const RADIANS_TO_DEGREE: f32 = 1 / DEGREE_TO_RADIANS;
pub const MODEL_TO_BLOCK_MULTIPLIER: f32 = 16.0;
pub const FRAME_EPSILON: f32 = 0.0001;
pub const FLOAT_COMPARISON_EPSILON: f32 = 1e-5f32;

pub const fn is_similar(a: f32, b: f32) -> bool {
    (a - b).abs() < FLOAT_COMPARISON_EPSILON
}

pub const fn similar_hash_code(a: f32) -> i32 {
    (((a / FLOAT_COMPARISON_EPSILON) as i32) as f32 * FLOAT_COMPARISON_EPSILON).to_bits() as i32
}

pub const VALID_ROTATION_DEGREES: [f32; 5] = [
    0.0,
    ROTATION_DEGREE,
    ROTATION_DEGREE * 2.0,
    -ROTATION_DEGREE,
    -ROTATION_DEGREE * 2.0,
];

pub fn valid_rotation_degree(rotation: f32) -> bool {
    VALID_ROTATION_DEGREES.contains(&rotation)
}

pub fn check_valid_degree(rotation: Float3) -> bool {
    let mut i = 0;
    if rotation.x == 0. {
        i += 1
    }
    if rotation.y == 0. {
        i += 1
    }
    if rotation.z == 0. {
        i += 1
    }
    i < 2
        && valid_rotation_degree(rotation.x)
        && valid_rotation_degree(rotation.y)
        && valid_rotation_degree(rotation.z)
}

pub fn animation_to_display(vector: Vector3<f32>) -> Vector3<f32> {
    Vector3::new(vector.x, -vector.y, -vector.z)
}

pub fn transform_to_display(vector: Vector3<f32>) -> Vector3<f32> {
    Vector3::new(vector.x, vector.y, -vector.z)
}
