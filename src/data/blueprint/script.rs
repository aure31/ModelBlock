use std::sync::Arc;
use std::sync::LazyLock;

use crate::data::raw::model::ModelAnimation;

use super::animation::AnimationType;

pub struct RenderSource {}

#[derive(Clone)]
pub struct AnimationScript {
    script: Arc<dyn Fn(RenderSource) + Send + Sync>,
}

impl AnimationScript {
    pub fn new<F>(script: F) -> Self
    where
        F: Fn(RenderSource) + Send + Sync + 'static,
    {
        Self {
            script: Arc::new(script),
        }
    }

    pub fn time(&self, time: f32) -> TimeScript {
        TimeScript {
            time,
            script: self.clone(),
        }
    }

    pub fn empty() -> &'static AnimationScript {
        static EMPTY: LazyLock<AnimationScript> = LazyLock::new(|| AnimationScript::new(|_e| {}));
        &EMPTY
    }
}

#[derive(Clone)]
pub struct TimeScript {
    time: f32,
    script: AnimationScript,
}

impl TimeScript {
    pub fn empty() -> Arc<TimeScript> {
        static EMPTY: LazyLock<Arc<TimeScript>> =
            LazyLock::new(|| Arc::new(AnimationScript::empty().time(0.0)));
        EMPTY.clone()
    }
}

pub struct BlueprintScript {
    name: String,
    typee: AnimationType,
    lenth: f32,
    scripts: Vec<Arc<TimeScript>>,
}

impl From<&ModelAnimation> for BlueprintScript {
    fn from(animation: &ModelAnimation) -> Self {
        Self {
            name: animation.name.clone(),
            typee: animation.looptype.clone(),
            lenth: animation.length,
            scripts: vec![
                TimeScript::empty(),
                Arc::new(AnimationScript::empty().time(animation.length)),
            ],
        }
    }
}
