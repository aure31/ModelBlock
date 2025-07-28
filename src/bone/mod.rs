use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;

use tokio::sync::Mutex;

// Ta structure BoneTag
#[derive(Clone, Copy, Debug)]
pub struct BoneTag {
    pub name: &'static str,
    pub tags: &'static [&'static str],
    pub item_mapper: Option<BoneItemMapper>,
}

// Juste un alias pour exemple, adapte selon ton code
pub type BoneItemMapper = fn();

// Constantes BoneTag
impl BoneTag {
    pub const HEAD: BoneTag = BoneTag::new("head", &["h"], None);
    pub const HEAD_WITH_CHILDREN: BoneTag = BoneTag::new("head_with_children", &["hi"], None);
    pub const HITBOX: BoneTag = BoneTag::new("hitbox", &["b", "ob"], None);
    pub const SEAT: BoneTag = BoneTag::new("seat", &["p"], None);
    pub const SUB_SEAT: BoneTag = BoneTag::new("sub_seat", &["sp"], None);

    pub const fn new(
        name: &'static str,
        tags: &'static [&'static str],
        item_mapper: Option<BoneItemMapper>,
    ) -> BoneTag {
        BoneTag {
            name,
            tags,
            item_mapper,
        }
    }
}

// Registry pour BoneTag
pub struct BoneTagRegistry {
    tags: HashMap<&'static str, BoneTag>,
}

impl BoneTagRegistry {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    pub fn register(&mut self, tag: BoneTag) {
        self.tags.insert(tag.name, tag);
    }

    pub fn get(&self, name: &str) -> Option<&BoneTag> {
        self.tags.get(name)
    }
}

// Singleton registry global
static BONE_TAG_REGISTRY: LazyLock<Arc<RwLock<BoneTagRegistry>>> =
    LazyLock::new(|| Arc::new(RwLock::new(BoneTagRegistry::new())));

fn get_registry() -> Arc<RwLock<BoneTagRegistry>> {
    BONE_TAG_REGISTRY.clone()
}

// Macro pour enregistrer plusieurs tags facilement
macro_rules! register_bone_tags {
    ( $( $tag:expr ),* $(,)? ) => {
        {
            let registry = get_registry();
            let mut w = registry.write().unwrap();
            $(
                w.register($tag);
            )*
        }
    };
}

// Fonction d'initialisation à appeler au démarrage
fn initialize_bone_tags() {
    register_bone_tags!(
        BoneTag::HEAD,
        BoneTag::HEAD_WITH_CHILDREN,
        BoneTag::HITBOX,
        BoneTag::SEAT,
        BoneTag::SUB_SEAT,
    );
}
