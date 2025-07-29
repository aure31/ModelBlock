use std::collections::HashMap;
use std::sync::Arc;

use crate::data::blueprint::animation::AnimationType;
use crate::utils::default_interpolation;
use crate::utils::VectorInterpolation;

use super::float3::Float3;
use super::float4::Float4;
use ordered_float::OrderedFloat;
use pumpkin_util::math::vector3::Vector3;
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ModelUV {
    uv: Float4,
    #[serde(default)]
    rotation: f32,
    #[serde(deserialize_with = "format_texture")]
    texture: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModelFace {
    up: ModelUV,
    down: ModelUV,
    north: ModelUV,
    south: ModelUV,
    west: ModelUV,
    east: ModelUV,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelElement {
    pub name: String,
    pub uuid: String,
    pub from: Float3,
    pub to: Float3,
    #[serde(default)]
    pub inflate: f32,
    #[serde(default)]
    pub rotation: Float3,
    pub origin: Float3,
    pub faces: ModelFace,
    #[serde(default)]
    pub visibility: bool,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl DataPoint {
    pub fn to_vector(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelKeyFrame {
    pub channel: KeyFrameChannel,
    pub data_points: Vec<DataPoint>,
    #[serde(default)]
    pub bezier_left_time: Float3,
    #[serde(default)]
    pub bezier_left_value: Float3,
    #[serde(default)]
    pub bezier_right_time: Float3,
    #[serde(default)]
    pub bezier_right_value: Float3,
    pub interpolation: Option<String>,
    pub uuid: String,
    pub time: f32,
}

impl ModelKeyFrame {
    pub fn find_interpolation(&self) -> Arc<dyn VectorInterpolation + Sync + Send + 'static> {
        if self.interpolation.is_none() {
            return default_interpolation();
        }
        todo!()
    }
}

impl PartialEq for ModelKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for ModelKeyFrame {}

impl PartialOrd for ModelKeyFrame {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ModelKeyFrame {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        OrderedFloat(self.time).cmp(&OrderedFloat(other.time))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelAnimator {
    pub name: Option<String>,
    pub keyframes: Vec<ModelKeyFrame>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelAnimation {
    pub name: String,
    #[serde(default)]
    pub looptype: AnimationType,
    #[serde(default)]
    pub overriding: bool,
    pub uuid: String,
    pub length: f32,
    pub animators: HashMap<String, ModelAnimator>,
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
