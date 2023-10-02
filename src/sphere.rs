use std::sync::Arc;

use crate::{hit::Hit, hit_record::HitRecord, material::Scatter, ray::Ray, vec3::Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Scatter>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}
impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().len_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.len_squared() - f64::powi(self.radius, 2);
        let disc = f64::powi(half_b, 2) - a * c;

        if disc < 0.00 {
            return None;
        }
        let sqrt = f64::sqrt(disc);
        let mut root = (-half_b - sqrt) / a;

        if root <= t_min || root >= t_max {
            root = (-half_b + sqrt) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let p = ray.at(root);
        let mut rec = HitRecord {
            point: p,
            normal: (p - self.center) / self.radius,
            t: root,
            front_face: false,
            material: self.material.clone(),
        };

        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
