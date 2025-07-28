use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use image::ImageBuffer;
use image::Rgba;
use pumpkin_util::math::vector3::Vector3;

use super::raw::float3::Float3;
use super::raw::model::AnimationType;
use super::raw::model::ModelChildren;
use super::raw::model::ModelData;
use super::raw::model::ModelElement;
use super::raw::model::ModelResolution;
use super::raw::model::ModelTexture;

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

pub struct RenderSource {}

pub struct AnimationScript {
    script: Arc<dyn Fn(RenderSource) -> ()>,
}

pub struct TimeScript {
    time: f32,
    script: AnimationScript,
}

pub struct BlueprintScript {
    name: String,
    typee: AnimationType,
    lenth: f32,
    scripts: Vec<TimeScript>,
}

enum BlueprintChildren {
    Element(ModelElement),
    Group(BlueprintGroup),
}

impl BlueprintChildren {
    pub fn from(value: ModelChildren, elements: &HashMap<String, ModelElement>) -> Self {
        match value {
            ModelChildren::Element(uuid) => {
                BlueprintChildren::Element(*elements.get(&uuid.uuid).unwrap())
            }
            ModelChildren::Group(group) => {
                let child = group.children.iter().map(|e| Self::from(*e, elements)).collect();
                BlueprintChildren::Group(BlueprintGroup{
                    name: BoneName::new(group.name),
                    origin: group.origin,
                    rotation: group.rotation,
                    children: child,
                })
            }
        }
    }
}

pub struct AnimationMovement {
    time: f32,
    tranform: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
}

pub struct BlueprintAnimator {
    name: String,
    key_frames: Vec<AnimationMovement>,
}

pub struct BlueprintAnimation {
    name: String,
    loop_type: AnimationType,
    lenth: f32,
    overriding: bool,
    animator: HashMap<BoneName, BlueprintAnimator>,
    script: BlueprintScript,
    empty_animator: Vec<AnimationMovement>,
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
    fn from(name: String, data: ModelData) -> Self {
        let group = 
        ModelBlueprint {
            name: data.name,
            scale: data.scale(),
            resolution: data.resolution,
            textures: data.textures.iter().map(|e| e.into()).collect(),
            //mapToList(data.outliner(), children -> BlueprintChildren.from(children, associate(data.elements(), ModelElement::uuid, e -> e)))
            group: data.outliner,
            animations: data.animations,
        }
    }
}
