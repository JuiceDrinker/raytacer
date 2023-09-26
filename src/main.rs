mod hit;
mod hit_record;
mod ray;
mod sphere;
mod util;
mod vec3;
mod world;

use std::{fs::File, io::Write};

use hit::Hit;

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::world::World;

fn ray_color(ray: &Ray, world: &dyn Hit) -> Vec3 {
    if let Some(hit_record) = world.hit(ray, 0.00, std::f64::INFINITY) {
        return 0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0));
    } else {
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: u32 = 600;
    let image_height = f64::max(1.00, image_width as f64 / ASPECT_RATIO) as u32;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.00, -viewport_height, 0.00);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.00, 0.00, focal_length) - viewport_u / 2.00 - viewport_v / 2.00;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut data = format!("P3\n{} {} \n255\n", image_width, image_height);

    let mut file = File::create("img.ppm").expect("Could not create file");

    for j in 0..image_height {
        for i in 0..image_width {
            log::debug!("\rScanlines lwefaeft: {}", image_height - j);

            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            data.push_str(&ray_color(&ray, &world).to_pixel_row());
        }
    }

    let _ = file.write_all(data.as_bytes());
    log::debug!("\rDone!       \n");
}
