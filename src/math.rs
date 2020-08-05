use raylib::math::{Vector2};
use serde::{Serialize, Deserialize};
use crate::traits::{ToVector2};

// Required since RayLib Vector2 can't be Deserialize
#[derive(Debug,Copy, Clone, Deserialize, Serialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

// Required since RayLib Vector2 can't be Deserialize
impl ToVector2 for Vec2 {
    fn to_vector2(&self) -> Vector2 {
        Vector2::from((self.x, self.y))
    }
}
