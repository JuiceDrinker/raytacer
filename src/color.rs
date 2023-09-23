use crate::vec3::Vec3;

pub struct Color {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Color {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3::new(x, y, z)
    }

    pub fn to_pixel_row(&self) -> String {
        format!(
            "{} {} {}\n",
            (&self.x * 255.999) as i32,
            (&self.y * 255.999) as i32,
            (&self.z * 255.999) as i32
        )
    }
}
