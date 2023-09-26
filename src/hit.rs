use crate::{hit_record::HitRecord, ray::Ray};

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
