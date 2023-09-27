use crate::{hit_record::HitRecord, ray::Ray, vec3::Vec3};

pub trait Scatter {
    fn scattter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scattter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_in_unit_sphere().unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Scatter for Metal {
    fn scattter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = ray_in.direction().reflect(hit_record.normal).unit_vector();
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
