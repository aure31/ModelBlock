use std::collections::HashMap;

use super::float3::Float3;
use super::float4::Float4;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ModelResolution {
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
pub struct ModelUV {
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
pub struct ModelElement {
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

impl ModelElement {
    pub fn max(&self) -> f32 {
        self.to.sub(&self.from).to_vec3().length()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelTexture {
    pub name: String,
    pub source: String,
    pub width: u32,
    pub height: u32,
    pub uv_width: u32,
    pub uv_height: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum AnimationType {
    #[default]
    PlayOnce,
    #[serde(rename = "loop")]
    Loop,
    #[serde(rename = "hold")]
    HoldOnLast,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum KeyFrameChannel {
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
pub struct DataPoint {
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
pub struct ModelKeyFrame {
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
pub struct ModelAnimator {
    name: String,
    keyframes: Vec<ModelKeyFrame>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelAnimation {
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
pub struct ModelGroupe {
    pub name: String,
    pub origin: Float3,
    pub rotation: Float3,
    pub uuid: String,
    pub children: Vec<ModelChildren>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelUUID {
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ModelChildren {
    Element(ModelUUID),
    Group(ModelGroupe),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ModelData {
    pub name: String,
    pub resolution: ModelResolution,
    pub elements: Vec<ModelElement>,
    pub outliner: Vec<ModelChildren>,
    pub textures: Vec<ModelTexture>,
    pub animations: Vec<ModelAnimation>,
}

impl ModelData {
    pub fn scale(&self) -> f32 {
        return self
            .elements
            .iter()
            .map(|e| e.max())
            .reduce(f32::max)
            .unwrap_or(16.0);
    }
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
