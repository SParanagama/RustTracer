use crate::math::*;

pub fn lerp(start: &Vec3, end: &Vec3, t: f64) -> Vec3 {
    (1.0 - t) * (*start) + t * (*end)
}
