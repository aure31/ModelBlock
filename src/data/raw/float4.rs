use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Float4 {
    dx: f32,
    dz: f32,
    tx: f32,
    ty: f32,
}
