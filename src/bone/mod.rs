use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::RwLock;

use tokio::sync::Mutex;

// Ta structure BoneTag
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    pub fn parse(&self, raw_name: &str) -> BoneName {
        let tag_array = raw_name.split("_").collect::<Vec<_>>();
        if tag_array.len() < 2 {
            return BoneName::new(HashSet::new(), raw_name.to_string(), raw_name.to_string());
        }
        let mut set: HashSet<BoneTag> = HashSet::new();
        for tag in &tag_array {
            if set.len() < tag_array.len()
                && let Some(tag) = self.get(tag)
            {
                set.insert(tag.clone());
            } else {
                let name = tag_array[set.len()..tag_array.len()].join("_");
                return BoneName::new(set, name, raw_name.to_string());
            }
        }
        BoneName::new(set, raw_name.to_string(), raw_name.to_string())
    }
}

// Singleton registry global
pub static BONE_TAG_REGISTRY: LazyLock<Arc<RwLock<BoneTagRegistry>>> =
    LazyLock::new(|| Arc::new(RwLock::new(BoneTagRegistry::new())));

pub fn get_registry() -> Arc<RwLock<BoneTagRegistry>> {
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

#[derive(Debug, Clone)]
pub struct BoneName {
    name: String,
    raw_name: String,
    tags: HashSet<BoneTag>,
}

impl Hash for BoneName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.raw_name.hash(state);
    }
}

impl PartialEq for BoneName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.raw_name == other.raw_name
    }
}

impl Eq for BoneName {}

impl BoneName {
    pub fn new(tags: HashSet<BoneTag>, name: String, raw_name: String) -> Self {
        Self {
            tags,
            name,
            raw_name,
        }
    }
}
