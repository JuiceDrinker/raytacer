use std::sync::Arc;

use crate::{material::Scatter, ray::Ray, vec3::Vec3};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub material: Arc<dyn Scatter>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.00;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal * -1.00
        };
    }
}
