use std::{fs::File, path::Path};

use tokio::io::BufReader;

struct ModelData {
    resolution: ModelResolution,
    elements: Vec<ModelElement>,
    outliner: Vec<ModelChildren>,
    textures: Vec<ModelTexture>,
    animations: Vec<ModelAnimation>,
}

impl ModelData {
    pub async fn load(path: &Path) -> Result<Self, ModelError> {
        let file = File::open(path).await.map_err(|e| ModelError::Io(e))?;
        let reader = BufReader::new(file);
        let model: Self = serde_json::from_reader(reader).map_err(|e| ModelError::Json(e))?;
        Ok(model)
    }
}
