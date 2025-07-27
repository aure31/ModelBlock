use std::collections::HashMap;

use crate::data::{Float3, Float4};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Default, Debug)]
struct ModelResolution {
    width: u32,
    height: u32,
}

fn format_texture<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u32::deserialize(deserializer)?;
    Ok(format!("#{}", value))
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ModelUV {
    uv: Float4,
    #[serde(default)]
    rotation: f32,
    #[serde(deserialize_with = "format_texture")]
    texture: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelFace {
    up: ModelUV,
    down: ModelUV,
    north: ModelUV,
    south: ModelUV,
    west: ModelUV,
    east: ModelUV,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelElement {
    name: String,
    uuid: String,
    from: Float3,
    to: Float3,
    #[serde(default)]
    inflate: f32,
    #[serde(default)]
    rotation: Float3,
    origin: Float3,
    faces: ModelFace,
    #[serde(default)]
    visibility: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelTexture {
    name: String,
    source: String,
    width: u32,
    height: u32,
    uv_width: u32,
    uv_height: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
enum AnimationType {
    #[default]
    PlayOnce,
    #[serde(rename = "loop")]
    Loop,
    #[serde(rename = "hold")]
    HoldOnLast,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum KeyFrameChannel {
    Position,
    Rotation,
    Scale,
    Timeline,
    Sound,
    Particle,
}

fn f32_from_str<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f32>().map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    #[serde(deserialize_with = "f32_from_str")]
    x: f32,
    #[serde(deserialize_with = "f32_from_str")]
    y: f32,
    #[serde(deserialize_with = "f32_from_str")]
    z: f32,
    #[serde(default)]
    script: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelKeyFrame {
    channel: KeyFrameChannel,
    data_points: Vec<DataPoint>,
    #[serde(default)]
    bezier_left_time: Float3,
    #[serde(default)]
    bezier_left_value: Float3,
    #[serde(default)]
    bezier_right_time: Float3,
    #[serde(default)]
    bezier_right_value: Float3,
    interpolation: String,
    uuid: String,
    time: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelAnimator {
    name: String,
    keyframes: Vec<ModelKeyFrame>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelAnimation {
    name: String,
    #[serde(default)]
    looptype: AnimationType,
    #[serde(default)]
    overriding: bool,
    uuid: String,
    length: f32,
    animators: HashMap<String, ModelAnimator>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelGroupe {
    name: String,
    origin: Float3,
    rotation: Float3,
    uuid: String,
    children: Vec<ModelChildren>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelUUID {
    uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ModelChildren {
    Element(ModelUUID),
    Group(ModelGroupe),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ModelData {
    resolution: ModelResolution,
    elements: Vec<ModelElement>,
    outliner: Vec<ModelChildren>,
    textures: Vec<ModelTexture>,
    animations: Vec<ModelAnimation>,
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;

    use super::ModelData;

    #[test]
    fn test_deserialize() {
        let model = ModelData::default();
        let json = serde_json::to_string(&model).unwrap();
        println!("{}", json);
    }

    #[test]
    fn test_serialize() {
        let file = File::open("test/test2.json").expect("failed to open file");
        let reader = BufReader::new(file);
        let model: ModelData = serde_json::from_reader(reader)
            .inspect_err(|e| println!("{}", e))
            .unwrap();
        println!("{:?}", model);
    }
}
