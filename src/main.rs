mod ray;
mod vec3;

use std::{fs::File, io::Write};

use crate::ray::Ray;
use crate::vec3::Vec3;

fn ray_color(ray: &Ray) -> Vec3 {
    let sphere = Vec3::new(0.00, 0.00, -1.00);
    let t = hit_sphere(&sphere, 0.5, ray);
    if t > 0.0 {
        let normal_vec = ray.at(t) - Vec3::new(0.0, 0.0, -1.0);
        return 0.5
            * Vec3::new(
                normal_vec.x() + 1.00,
                normal_vec.y() + 1.00,
                normal_vec.z() + 1.00,
            );
    }
    let direction_vector = &ray.direction();
    let unit_vector = direction_vector.unit_vector();
    let a = 0.5 * (unit_vector.y() + 1.0);
    (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(&center: &Vec3, radius: f32, &ray: &Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = ray.direction().len_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.len_squared() - f32::powi(radius, 2);
    let disc = f32::powi(half_b, 2) - a * c;

    if disc < 0.00 {
        -1.0
    } else {
        (-half_b - f32::sqrt(disc)) / (a)
    }
}
fn main() {
    // Image

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    let image_width: u32 = 600;
    let image_height = f32::max(1.00, image_width as f32 / ASPECT_RATIO) as u32;

    // Camera

    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * image_width as f32 / image_height as f32;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.00, -viewport_height, 0.00);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0.00, 0.00, focal_length) - viewport_u / 2.00 - viewport_v / 2.00;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut data = format!("P3\n{} {} \n255\n", image_width, image_height);

    let mut file = File::create("img.ppm").expect("Could not create file");

    for j in 0..image_height {
        for i in 0..image_width {
            log::debug!("\rScanlines lwefaeft: {}", image_height - j);

            let pixel_center = pixel00_loc + (pixel_delta_u * i as f32) + pixel_delta_v * j as f32;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            data.push_str(&ray_color(&ray).to_pixel_row());
        }
    }

    let _ = file.write_all(data.as_bytes());
    log::debug!("\rDone!       \n");
}
