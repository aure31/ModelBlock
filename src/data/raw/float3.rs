use pumpkin_util::math::vector3::Vector3;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    const CENTER: Float3 = Float3::new(8., 8., 8.);

    pub fn to_vec3(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }

    pub const fn flat(n: f32) -> Self {
        Self::new(n, n, n)
    }

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub async fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub async fn mul(&self, other: &Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub async fn div(&self, other: &Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }

    pub async fn invert_xz(&self) -> Self {
        Self::new(-self.z, self.y, -self.x)
    }

    pub async fn to_block_scale(&self) -> Self {
        self.div(&Self::flat(16.0)).await
    }
}
