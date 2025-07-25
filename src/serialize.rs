use std::collections::HashMap;

use crate::data::{Float3, Float4};
use serde::Deserialize;

#[derive(Deserialize)]
struct ModelResolution {
    width: u32,
    height: u32,
}

#[derive(Deserialize)]
struct ModelUV {
    uv: Float4,
    rotation: f32,
    texture: String,
}

#[derive(Deserialize)]
struct ModelFace {
    up: ModelUV,
    down: ModelUV,
    north: ModelUV,
    south: ModelUV,
    west: ModelUV,
    east: ModelUV,
}

#[derive(Deserialize)]
struct ModelElement {
    name: String,
    uuid: String,
    from: Float3,
    to: Float3,
    inflate: f32,
    rotation: Float3,
    origin: Float3,
    faces: ModelFace,
    visibility: bool,
}

#[derive(Deserialize)]
struct ModelTexture {
    name: String,
    source: String,
    width: u32,
    height: u32,
    uv_width: u32,
    uv_height: u32,
}

#[derive(Deserialize)]
enum AnimationType {
    PlayOnce,
    Loop,
    HoldOnLast,
}

#[derive(Deserialize)]
enum KeyFrameChannel {
    Position,
    Rotation,
    Scale,
    Timeline,
    Sound,
    Particle,
}

#[derive(Deserialize)]
struct DataPoint {
    x: f32,
    y: f32,
    z: f32,
    script: String,
}

#[derive(Deserialize)]
struct ModelKeyFrame {
    channel: KeyFrameChannel,
    data_points: Vec<DataPoint>,
    bezier_left_time: Float3,
    bezier_left_value: Float3,
    bezier_right_time: Float3,
    bezier_right_value: Float3,
    interpolation: String,
    time: f32,
}

#[derive(Deserialize)]
struct ModelAnimator {
    name: String,
    keyframes: Vec<ModelKeyFrame>,
}

#[derive(Deserialize)]
struct ModelAnimation {
    name: String,
    looptype: AnimationType,
    overriding: bool,
    uuid: String,
    lenth: f32,
    animators: HashMap<String, ModelAnimator>,
}

#[derive(Deserialize)]
struct ModelGroupe {
    name: String,
    origin: Float3,
    rotation: Float3,
    uuid: String,
    children: Vec<ModelChildren>,
}

struct ModelUUID {
    uuid: String,
}

#[derive(Deserialize)]
enum ModelChildren {
    Group(ModelGroupe),
    Element(ModelElement),
}

#[derive(Deserialize)]
struct ModelData {
    resolution: ModelResolution,
    elements: Vec<ModelElement>,
    outliner: Vec<ModelChildren>,
    textures: Vec<ModelTexture>,
    animations: Vec<ModelAnimation>,
}
