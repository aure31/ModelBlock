use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::LazyLock;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use image::ImageBuffer;
use image::Rgba;
use pumpkin_util::math::vector3::Vector3;

use crate::bone::BoneName;
use crate::bone::get_registry;

use self::animation::BlueprintAnimation;

use super::raw::float3::Float3;
use super::raw::model::ModelAnimation;
use super::raw::model::ModelChildren;
use super::raw::model::ModelData;
use super::raw::model::ModelElement;
use super::raw::model::ModelResolution;
use super::raw::model::ModelTexture;

pub mod animation;
pub mod script;

pub struct BlueprintTexture {
    name: String,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    uv_width: i32,
    uv_height: i32,
}

impl From<&ModelTexture> for BlueprintTexture {
    fn from(value: &ModelTexture) -> Self {
        let base64_part = &value
            .source
            .split(',')
            .nth(1)
            .expect("Invalid data URL format");

        let decoded = BASE64_STANDARD
            .decode(base64_part)
            .expect("Base64 decode failed");
        Self {
            name: value.name.clone(),
            image: image::load_from_memory(&decoded)
                .expect("Failed to load image")
                .into_rgba8(),
            uv_width: value.uv_width as i32,
            uv_height: value.uv_height as i32,
        }
    }
}

pub struct BlueprintGroup {
    name: BoneName,
    origin: Float3,
    rotation: Float3,
    children: Vec<BlueprintChildren>,
}

enum BlueprintChildren {
    Element(ModelElement),
    Group(BlueprintGroup),
}

impl BlueprintChildren {
    pub fn from(value: &ModelChildren, elements: &HashMap<String, ModelElement>) -> Self {
        match value {
            ModelChildren::Element(uuid) => {
                BlueprintChildren::Element(elements.get(&uuid.uuid).unwrap().clone())
            }
            ModelChildren::Group(group) => {
                let child = group
                    .children
                    .iter()
                    .map(|e| Self::from(e, elements))
                    .collect();
                BlueprintChildren::Group(BlueprintGroup {
                    name: get_registry().read().unwrap().parse(&group.name),
                    origin: group.origin.clone(),
                    rotation: group.rotation.clone(),
                    children: child,
                })
            }
        }
    }
}

struct ModelBlueprint {
    name: String,
    scale: f32,
    resolution: ModelResolution,
    textures: Vec<BlueprintTexture>,
    group: Vec<BlueprintChildren>,
    animations: HashMap<String, BlueprintAnimation>,
}

impl From<ModelData> for ModelBlueprint {
    fn from(data: ModelData) -> Self {
        let elements: HashMap<String, ModelElement> = data
            .elements
            .iter()
            .map(|e| (e.uuid.clone(), e.clone()))
            .collect();
        let group = data
            .outliner
            .iter()
            .map(|e| BlueprintChildren::from(e, &elements))
            .collect();

        ModelBlueprint {
            name: data.name,
            scale: data.scale(),
            resolution: data.resolution,
            textures: data.textures.iter().map(|e| e.into()).collect(),
            animations: data.animations.iter().map(|e| e.to),
            group: group,
        }
    }
}
