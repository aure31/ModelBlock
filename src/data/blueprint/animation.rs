use std::collections::BTreeSet;
use std::collections::HashMap;
use std::mem::take;
use std::sync::Arc;
use std::sync::Weak;

use ordered_float::OrderedFloat;
use pumpkin_util::math::vector3::Math;
use pumpkin_util::math::vector3::Vector3;
use serde::Deserialize;
use serde::Serialize;

use crate::bone::BoneName;
use crate::bone::BONE_TAG_REGISTRY;
use crate::data::blueprint::BlueprintChildren;
use crate::data::raw::model::KeyFrameChannel;
use crate::data::raw::model::ModelAnimation;
use crate::data::raw::model::ModelKeyFrame;
use crate::utils::math;
use crate::utils::sum;
use crate::utils::VectorInterpolation;
use crate::utils::VectorPoint;

use super::script::BlueprintScript;
use super::script::TimeScript;
use super::BlueprintGroup;

#[derive(Clone,Default)]
pub struct AnimationMovement {
    time: f32,
    tranform: Vector3<f32>,
    scale: Vector3<f32>,
    rotation: Vector3<f32>,
}

impl AnimationMovement {
    pub fn new(time: f32, tranform: Vector3<f32>, scale: Vector3<f32>, rotation: Vector3<f32>) -> Self {
        Self {
            time,
            tranform,
            scale,
            rotation,
        }
    }

    pub fn from_lenght(length: f32) -> Self {
        Self {
            time: length,
            ..Default::default()
        }
    }
}

pub struct BlueprintAnimator {
    pub name: String,
    pub key_frame: Vec<AnimationMovement>,
}

impl BlueprintAnimator {
    pub const fn builder(length: f32) -> BlueprintAnimatorBuilder {
        BlueprintAnimatorBuilder::new(length)
    }
}

pub struct AnimatorData {
    pub name: String,
    pub points: Vec<AnimationPoint>,
}

pub struct BlueprintAnimatorBuilder {
    length: f32,
    transform: Vec<VectorPoint>,
    scale: Vec<VectorPoint>,
    rotation: Vec<VectorPoint>,
}

impl BlueprintAnimatorBuilder {
    pub const fn new(length: f32) -> Self {
        Self {
            length,
            transform: Vec::new(),
            scale: Vec::new(),
            rotation: Vec::new(),
        }
    }

    pub fn add_frame(&mut self, keyframe: &ModelKeyFrame) -> &mut Self {
        if keyframe.time > self.length {
            return self;
        }

        let interpolation: Arc<dyn VectorInterpolation + Sync + Send + 'static> =
            keyframe.find_interpolation();
        for data_point in &keyframe.data_points {
            let vec = data_point.to_vector();
            let div = |a: Vector3<f32>, b: f32| Vector3::new(a.x / b, a.y / b, a.z / b);
            match keyframe.channel {
                KeyFrameChannel::Position => self.transform.push(VectorPoint::new(
                    math::transform_to_display(div(vec, math::MODEL_TO_BLOCK_MULTIPLIER)),
                    keyframe.time,
                    interpolation.clone(),
                )),
                KeyFrameChannel::Rotation => self.rotation.push(VectorPoint::new(
                    math::animation_to_display(vec),
                    keyframe.time,
                    interpolation.clone(),
                )),
                KeyFrameChannel::Scale => self.scale.push(VectorPoint::new(
                    vec.sub(&Vector3::new(1.0, 1.0, 1.0)),
                    keyframe.time,
                    interpolation.clone(),
                )),
                _ => {}
            }
        }

        self
    }

    pub fn build(&mut self, name: impl Into<String>) -> AnimatorData {
        AnimatorData {
            name: name.into(),
            points: sum(
                self.length,
                &unique(take(&mut self.transform)),
                &unique(take(&mut self.rotation)),
                &unique(take(&mut self.scale)),
            ),
        }
    }
}

fn unique(items: Vec<VectorPoint>) -> Vec<VectorPoint> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();

    items
        .into_iter() // itère par valeur, ownership pris
        .filter(|vp| seen.insert(OrderedFloat(vp.time))) // garde si temps jamais vu
        .collect()
}

impl BlueprintAnimator {
    pub fn iterator(&self, typ: AnimationType) -> Arc<dyn AnimationIterator> {
        typ.create(self.key_frame.iter().map(|x| x.clone().into()).collect())
    }
}

trait AnimationIterator: Iterator<Item = Timed> {
    fn r#type(&self) -> AnimationType;
}

struct PlayOnce {
    key_frame: Vec<Timed>,
    index: usize,
}

impl AnimationIterator for PlayOnce {
    fn r#type(&self) -> AnimationType {
        todo!()
    }
}

impl Iterator for PlayOnce {
    type Item = Timed;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.key_frame.len() {
            return None;
        }
        let item = self.key_frame[self.index].clone();
        self.index += 1;
        Some(item)
    }
}

struct Loop {
    key_frame: Vec<Timed>,
    index: usize,
}

impl AnimationIterator for Loop {
    fn r#type(&self) -> AnimationType {
        todo!()
    }
}

impl Iterator for Loop {
    type Item = Timed;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.key_frame.len() {
            self.index = 1;
        }
        let item = self.key_frame[self.index].clone();
        self.index += 1;
        Some(item)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum AnimationType {
    #[default]
    PlayOnce,
    #[serde(rename = "loop")]
    Loop,
    #[serde(rename = "hold")]
    HoldOnLast,
}

#[derive(Clone)]
pub enum Timed {
    Script(TimeScript),
    KeyFrame(AnimationMovement),
}

impl From<AnimationMovement> for Timed {
    fn from(value: AnimationMovement) -> Self {
        Self::KeyFrame(value)
    }
}

impl From<TimeScript> for Timed {
    fn from(value: TimeScript) -> Self {
        Self::Script(value)
    }
}

impl AnimationType {
    fn create(&self, key_frames: Vec<Timed>) -> Arc<dyn AnimationIterator> {
        match self {
            AnimationType::PlayOnce => Arc::new(PlayOnce {
                key_frame: key_frames.clone(),
                index: 0,
            }),
            AnimationType::Loop => todo!(),
            AnimationType::HoldOnLast => todo!(),
        }
    }
}

pub struct BlueprintAnimation {
    name: String,
    loop_type: AnimationType,
    lenth: f32,
    overriding: bool,
    animator: HashMap<BoneName, BlueprintAnimator>,
    script: Option<BlueprintScript>,
    empty_animator: Vec<AnimationMovement>,
}

impl BlueprintAnimation {
    fn from(children: &BlueprintGroup, animation: &ModelAnimation) -> Self {
        let mut map: HashMap<BoneName, AnimatorData> = HashMap::new();
        let mut blueprint_script: Option<BlueprintScript> = if animation.overriding {
            None
        } else {
            Some(animation.into())
        };
        let animator = animation.animators;
        for (uuid, animator) in animator {
            let name = if let Some(name) = animator.name {
                name
            } else {
                continue;
            };
            let mut builder = BlueprintAnimator::builder(animation.length);
            let mut keyframes = animator.keyframes.clone();
            keyframes.sort();
            for keyframe in keyframes {
                builder.add_frame(&keyframe);
            }
            if uuid == "effect" {
                blueprint_script = Some(BlueprintScript::from(animation))
            } else {
                map.insert(
                    BONE_TAG_REGISTRY.read().unwrap().parse(&name),
                    builder.build(name),
                );
            }
        }
        let animators : HashMap<BoneName, BlueprintAnimator> = AnimationGenerator::createMovements(children, map);
        let empty_animator = if animators.is_empty() {
            vec![AnimationMovement::default(), AnimationMovement::from_lenght(animation.length)]
        } else {
            vec![]
        }
        Self {
            name: animation.name,
            loop_type: animation.looptype,
            lenth: animation.length,
            overriding: animation.overriding,
            animator: animators,
            script: blueprint_script,
            empty_animator: ,
        }
    }
}

pub struct AnimationTree {
    pub parent: Weak<AnimationTree>, // lien faible vers le parent
    pub children: Vec<Arc<AnimationTree>>, // enfants avec Arc
    pub points: Vec<AnimationPoint>,
}

impl AnimationTree {
    pub fn new_root(
        point_map: &HashMap<BoneName, AnimatorData>,
        group: &BlueprintGroup,
        points: Vec<AnimationPoint>,
    ) -> Arc<Self> {
        // Création de la racine sans parent
        let root = Arc::new(Self {
            parent: Weak::new(),
            children: vec![],
            points,
        });

        // Ajout récursif des enfants
        let children = Self::build_children(point_map, &root, group);
        // On remplit les enfants de la racine
        Arc::get_mut(&mut Arc::clone(&root))
            .unwrap()
            .children = children;

        root
    }

    fn build_children(
        point_map: &HashMap<BoneName, AnimatorData>,
        parent: &Arc<AnimationTree>,
        group: &BlueprintGroup,
    ) -> Vec<Arc<AnimationTree>> {
        group
            .children
            .iter()
            .filter_map(|g| {
                if let BlueprintChildren::Group(b) = g {
                    // Récupération éventuelle des points
                    let points = point_map
                        .get(&b.name)
                        .map(|a| a.points.clone())
                        .unwrap_or_default();

                    // Création de l’enfant avec Weak vers le parent
                    let child = Arc::new(Self {
                        parent: Arc::downgrade(parent),
                        children: vec![],
                        points,
                    });

                    // Construction récursive des enfants de cet enfant
                    let sub_children =
                        Self::build_children(point_map, &child, b);

                    // Ajout des enfants à l’enfant
                    Arc::get_mut(&mut Arc::clone(&child))
                        .unwrap()
                        .children = sub_children;

                    Some(child)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn flatten_leaves(self: &Arc<Self>) -> Vec<Arc<AnimationTree>> {
        if self.children.is_empty() {
            // C'est une feuille → on la garde
            vec![self.clone()]
        } else {
            // Sinon on explore seulement les enfants
            self.children
                .iter()
                .flat_map(|child| child.flatten_leaves())
                .collect()
        }
    }}

pub struct AnimationGenerator {
    point_map: HashMap<BoneName, AnimatorData>,
    trees: Vec<Arc<AnimationTree>>,
}

impl AnimationGenerator {
    pub fn new(point_map: HashMap<BoneName, AnimatorData>, children: Vec<BlueprintChildren>) -> Self {
        let trees : Vec<Arc<AnimationTree>> = children
            .iter()
            .filter_map(|g| {
                if let BlueprintChildren::Group(b) = g {
                    Some(AnimationTree::new_root(&point_map, b, vec![]))
                } else {
                    None
                }
            })
            .flat_map(|t| t.flatten_leaves().clone())
            .collect();
        Self {
            point_map,
            trees,
        }

    }

    pub fn create_movements(group: &BlueprintGroup, point_map: HashMap<BoneName, AnimatorData>) -> HashMap<BoneName, BlueprintAnimator> {
        let floatset : BTreeSet<OrderedFloat<f32>> = point_map.values().flat_map(|a| a.points.clone()).map(|p| OrderedFloat(p.position.time)).collect(); 
        
        todo!()

    }
}



#[derive(Clone)]
pub struct AnimationPoint {
    pub position: VectorPoint,
    pub rotation: VectorPoint,
    pub scale: VectorPoint,
}

