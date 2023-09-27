mod camera;
mod hit;
mod hit_record;
mod material;
mod ray;
mod sphere;
mod util;
mod vec3;
mod world;

use std::io::stderr;
use std::io::Write;
use std::rc::Rc;

use hit::Hit;
use rand::Rng;

use crate::camera::Camera;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use crate::world::World;

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(hit_record) = world.hit(ray, 0.001, std::f64::INFINITY) {
        if let Some((a, s)) = hit_record.material.scattter(ray, &hit_record) {
            a * ray_color(&s, world, depth - 1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 300;
    const MAX_DEPTH: u64 = 50;
    // World
    let mut world = World::new();
    let mat_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_right = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right);
    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    // Camera

    let cam = Camera::new();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let mut rng = rand::thread_rng();
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();
                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH)
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL))
        }
    }

    log::debug!("\rDone!");
}
