use crate::{hit_record::HitRecord, ray::Ray};

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
