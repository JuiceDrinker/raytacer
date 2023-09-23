use super::vec3::Vec3;
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3::new(x, y, z)
    }
}
